use {
    super::binary,
    crate::{
        binary::{MpvBinary, get_mpv_binary},
        ipc::{
            self,
            event_bus::{EventBus, split::BusEventsHalf},
        },
        protocol::event::{
            MpvEvent,
            grouped_events::{FileEvent, PlaybackControlEvent},
        },
    },
    futures::{FutureExt, Stream, TryFutureExt},
    futures_util::StreamExt,
    std::{
        future::ready,
        path::Path,
        process::{ExitStatus, Stdio},
    },
    tap::{Pipe, Tap},
    tokio::{
        net::UnixStream,
        sync::oneshot,
        time::{Duration, sleep},
    },
    tracing::{Instrument, Span, debug, debug_span, info, instrument, trace, warn},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not spawn an instance")]
    SpawningInstance,
    #[error("Getting binary")]
    GettingBinary(#[source] binary::Error),
    #[error("Spawning anonymous pipe")]
    SpawningPipe(#[source] std::io::Error),
    #[error("Spawning mpv command")]
    SpawningCommand(#[source] std::io::Error),
    #[error("Waiting for mpv process to finish")]
    WaitingForFinish(#[source] std::io::Error),
    #[error("Connecting to socket for IPC")]
    ConnectingToSocket(#[source] std::io::Error),
    #[error("MPV command ended with bad status code: {code:?}")]
    BadStatusCode { code: ExitStatus },
    #[error("Could not create a tempdir for socket")]
    CreatingTempDir(#[source] std::io::Error),
    #[error("Could not wait for initialisation due to closed ipc stream")]
    StreamClosedDuringPlaybackWait,
    #[error("Error occurred while waiting for file loaded event")]
    WaitingForPlaybackStart(#[source] ipc::Error),
    #[error("Error occurred when killing process")]
    KillingProcess(#[source] std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct MpvInstance {
    pub process: MpvProcess,
}

#[derive(Debug)]
pub struct WatchedCommand {
    finish: Option<oneshot::Receiver<std::io::Result<ExitStatus>>>,
    kill: Option<oneshot::Sender<()>>,
    #[allow(dead_code)]
    join: tokio::task::JoinHandle<()>,
}

impl Drop for WatchedCommand {
    fn drop(&mut self) {
        let status = self.blocking_kill().expect("dropping failed");
        debug!("[DROP] process exited with {status:?}")
    }
}

impl WatchedCommand {
    pub async fn wait(&mut self) -> std::io::Result<ExitStatus> {
        self.finish
            .take()
            .expect("already awaited")
            .await
            .expect("could not communicate with the process")
    }
    /// TODO
    pub(crate) fn blocking_kill(&mut self) -> std::io::Result<ExitStatus> {
        Ok(ExitStatus::default())
        // if let Err(reason) = self.kill.send(()) {
        //     tracing::debug!("could not communicate kill with to process: {reason:?}");
        //     Ok(ExitStatus::default())
        // } else {
        //     self.finish
        //         .take()
        //         .expect("already")
        //         .expect("could not communicate post kill")
        // }
    }

    pub(crate) async fn kill(&mut self) -> std::io::Result<ExitStatus> {
        match self.kill.take() {
            Some(kill) => {
                if let Err(reason) = kill.send(()) {
                    tracing::debug!("could not communicate kill with to process: {reason:?}");
                    Ok(ExitStatus::default())
                } else {
                    self.finish
                        .take()
                        .expect("already killed")
                        .await
                        .expect("could not communicate post kill")
                }
            }
            None => Ok(ExitStatus::default()),
        }
    }

    pub async fn spawn(mut command: tokio::process::Command) -> std::io::Result<Self> {
        let (finish_tx, finish_rx) = oneshot::channel();
        let (kill_tx, mut kill_rx) = oneshot::channel::<()>();
        let (could_not_spawn_tx, could_not_spawn_rx) = oneshot::channel();
        let join = tokio::task::spawn(async move {
            (match command.spawn() {
                Ok(child) => {
                    could_not_spawn_tx.send(Ok(())).unwrap();
                    Ok(child)
                }
                Err(error) => {
                    could_not_spawn_tx.send(Err(error)).unwrap();
                    sleep(Duration::from_millis(50)).await;
                    panic!("could not spawn process")
                }
            })
            .pipe(ready)
            .and_then(async |mut child| {
                loop {
                    sleep(Duration::from_millis(1000 / 60)).await;
                    match child.try_wait() {
                        Ok(maybe_exit) => match maybe_exit {
                            Some(exit) => {
                                info!("process exited with {exit:?}");
                                return Ok(child);
                            }
                            None => match kill_rx.try_recv() {
                                Ok(()) => child.kill().await.expect("explicit kill: killing child"),
                                Err(e) => match e {
                                    oneshot::error::TryRecvError::Empty => continue,
                                    oneshot::error::TryRecvError::Closed => child.kill().await.expect("closed: killing child"),
                                },
                            },
                        },
                        Err(e) => return Err(e),
                    }
                }
            })
            .and_then(async |mut child| child.wait().await)
            .then(async |out| {
                finish_tx
                    .send(out.tap(|finished| warn!("finished:{finished:?}")))
                    .expect("failed to communicate exit")
            })
            .await
        });
        could_not_spawn_rx
            .await
            .expect("could not communicate with thread")
            .map(|_| Self {
                finish: Some(finish_rx),
                kill: Some(kill_tx),
                join,
            })
    }
}

#[derive(Debug)]
pub struct MpvProcess {
    #[allow(dead_code)]
    binary: MpvBinary<'static>,
    #[allow(dead_code)]
    pub child: WatchedCommand,
    #[allow(dead_code)]
    socket_file: tempfile::NamedTempFile,
    pub event_bus: EventBus,
}

impl BusEventsHalf {
    pub fn events(&mut self) -> impl Stream<Item = MpvEvent> + '_ {
        &mut self.events
    }
}

impl MpvProcess {
    pub async fn kill(mut self) -> Result<ExitStatus> {
        self.child.kill().await.map_err(Error::KillingProcess)
    }
    #[instrument(err, level = "TRACE")]
    async fn spawn(media_path: &Path) -> Result<Self> {
        let ipc_socket_path = tempfile::NamedTempFile::new().map_err(Error::CreatingTempDir)?;
        get_mpv_binary()
            .map_err(self::Error::GettingBinary)
            .pipe(ready)
            .and_then(async |binary| {
                binary
                    .command()
                    .pipe(async |command| -> Result<_> {
                        command
                            .tap_mut(|binary| {
                                binary
                                    .arg(format!("--input-ipc-server={}", ipc_socket_path.path().display()))
                                    .arg(media_path)
                                    .stdin(Stdio::null())
                                    .stdout(Stdio::null())
                                    .stderr(Stdio::null());
                            })
                            .tap(|command| {
                                debug!("running command {command:?}");
                            })
                            .pipe(WatchedCommand::spawn)
                            .map_err(Error::SpawningCommand)
                            .and_then(async |child| {
                                let connect = async || {
                                    for _ in 0..1000 {
                                        match UnixStream::connect(&ipc_socket_path)
                                            .await
                                            .map_err(Error::ConnectingToSocket)
                                        {
                                            Ok(s) => return Ok(s),
                                            Err(reason) => {
                                                trace!("[RECONNECT] {reason:?}");
                                                sleep(Duration::from_millis(5)).await;
                                                continue;
                                            }
                                        }
                                    }
                                    UnixStream::connect(&ipc_socket_path)
                                        .await
                                        .map_err(Error::ConnectingToSocket)
                                };
                                connect().await.map(|ipc_stream| Self {
                                    binary,
                                    child,
                                    socket_file: ipc_socket_path,
                                    event_bus: EventBus::spawn(ipc_stream),
                                })
                            })
                            .await
                    })
                    .await
            })
            .await
    }
}

impl BusEventsHalf {
    #[instrument(skip(self))]
    pub async fn await_playback(&mut self) -> Result<()> {
        let mut await_event = async |event| {
            let _s = debug_span!("awaiting event", ?event).entered();
            trace!("waiting");
            self.events()
                .filter_map(|ev| (ev == event).then_some(()).pipe(ready))
                .next()
                .instrument(Span::current())
                .await
                .ok_or(Error::StreamClosedDuringPlaybackWait)
        };
        await_event(MpvEvent::File(FileEvent::FileLoaded)).await?;
        await_event(MpvEvent::PlaybackControl(PlaybackControlEvent::PlaybackRestart)).await?;

        Ok(())
    }
}

impl MpvInstance {
    #[instrument]
    pub async fn new(media_path: &Path) -> Result<Self> {
        MpvProcess::spawn(media_path)
            .map_ok(|process| Self { process })
            .await
    }

    #[instrument(skip(self))]
    pub async fn finish(mut self) -> Result<()> {
        self.process
            .child
            .wait()
            .await
            .map_err(Error::WaitingForFinish)
            .and_then(|code| if code.success() { Ok(()) } else { Err(Error::BadStatusCode { code }) })
    }
}
