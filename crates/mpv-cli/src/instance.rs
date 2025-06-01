use {
    super::binary,
    crate::{
        binary::{MpvBinary, get_mpv_binary},
        ipc::{self, HandledIpcResponse},
        protocol::event::{
            MpvEvent,
            MpvEventKind,
            grouped_events::{FileEvent, PlaybackControlEvent},
        },
    },
    postage::{
        sink::Sink,
        stream::{Stream, TryRecvError},
    },
    ringbuf::{HeapRb, LocalRb},
    std::{
        collections::BTreeMap,
        convert::identity,
        io::BufReader,
        os::unix::net::UnixStream,
        path::Path,
        process::{Command, ExitStatus, Stdio},
        thread::sleep,
        time::Duration,
    },
    tap::{Pipe, Tap},
    tracing::{debug, info, instrument, trace},
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
    StreamClosed,
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
    #[derivative(Debug = "ignore")]
    buffer: HeapRb<HandledIpcResponse<serde_json::Value>>,
    pub(crate) line_buffer: String,
}

#[derive(Debug)]
struct WatchedCommand {
    finish: postage::oneshot::Receiver<std::io::Result<ExitStatus>>,
    kill: postage::oneshot::Sender<()>,
    #[allow(dead_code)]
    join: std::thread::JoinHandle<()>,
}

impl Drop for WatchedCommand {
    fn drop(&mut self) {
        let status = self.kill().expect("dropping failed");
        debug!("[DROP] process exited with {status:?}")
    }
}

impl WatchedCommand {
    pub fn wait(mut self) -> std::io::Result<ExitStatus> {
        self.finish
            .blocking_recv()
            .expect("could not communicate with the process")
    }
    pub fn kill(&mut self) -> std::io::Result<ExitStatus> {
        if let Err(reason) = self.kill.blocking_send(()) {
            tracing::debug!("could not communicate kill with to process: {reason:?}");
            Ok(ExitStatus::default())
        } else {
            self.finish
                .blocking_recv()
                .expect("could not communicate post kill")
        }
    }
    pub fn spawn(mut command: Command) -> std::io::Result<Self> {
        let (mut finish_tx, finish_rx) = postage::oneshot::channel();
        let (kill_tx, mut kill_rx) = postage::oneshot::channel::<()>();
        let (mut could_not_spawn_tx, mut could_not_spawn_rx) = postage::oneshot::channel();
        let join = std::thread::spawn(move || {
            (match command.spawn() {
                Ok(child) => {
                    could_not_spawn_tx.blocking_send(Ok(())).unwrap();
                    Ok(child)
                }
                Err(error) => {
                    could_not_spawn_tx.blocking_send(Err(error)).unwrap();
                    sleep(Duration::from_millis(50));
                    panic!("could not spawn process")
                }
            })
            .and_then(|mut child| {
                loop {
                    sleep(Duration::from_millis(1000 / 60));
                    match child.try_wait() {
                        Ok(maybe_exit) => match maybe_exit {
                            Some(exit) => {
                                info!("process exited with {exit:?}");
                                return Ok(child);
                            }
                            None => match kill_rx.try_recv() {
                                Ok(()) => child.kill().expect("explicit kill: killing child"),
                                Err(e) => match e {
                                    TryRecvError::Pending => continue,
                                    TryRecvError::Closed => child.kill().expect("closed: killing child"),
                                },
                            },
                        },
                        Err(e) => return Err(e),
                    }
                }
            })
            .and_then(|mut child| child.wait())
            .pipe(|out| {
                finish_tx
                    .blocking_send(out)
                    .expect("failed to communicate exit")
            })
        });
        could_not_spawn_rx
            .blocking_recv()
            .expect("could not communicate with thread")
            .map(|_| Self {
                finish: finish_rx,
                kill: kill_tx,
                join,
            })
    }
}

#[derive(Debug)]
pub struct MpvProcess {
    #[allow(dead_code)]
    binary: MpvBinary<'static>,
    #[allow(dead_code)]
    child: WatchedCommand,
    #[allow(dead_code)]
    socket_file: tempfile::NamedTempFile,
    pub ipc_stream: BufReader<UnixStream>,
}

impl MpvProcess {
    pub fn kill(mut self) -> Result<ExitStatus> {
        self.child.kill().map_err(Error::KillingProcess)
    }
    #[instrument(ret, level = "TRACE")]
    fn spawn(media_path: &Path) -> Result<Self> {
        let ipc_socket_path = tempfile::NamedTempFile::new().map_err(Error::CreatingTempDir)?;
        get_mpv_binary()
            .map_err(self::Error::GettingBinary)
            .and_then(|binary| {
                binary.command().pipe(|command| -> Result<_> {
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
                        .and_then(|child| {
                            let connect = || {
                                for _ in 0..1000 {
                                    match UnixStream::connect(&ipc_socket_path).map_err(Error::ConnectingToSocket) {
                                        Ok(s) => return Ok(s),
                                        Err(reason) => {
                                            trace!("{reason:?}");
                                            sleep(Duration::from_millis(2));
                                            continue;
                                        }
                                    }
                                }
                                UnixStream::connect(&ipc_socket_path).map_err(Error::ConnectingToSocket)
                            };
                            connect().map(BufReader::new).map(|ipc_stream| Self {
                                binary,
                                child,
                                socket_file: ipc_socket_path,
                                ipc_stream,
                            })
                        })
                })
            })
    }
}

impl MpvInstance {
    pub fn new(media_path: &Path) -> Result<Self> {
        MpvProcess::spawn(media_path).map(|process| Self {
            process,
            buffer: HeapRb::new(1024),
            line_buffer: String::new(),
        })
    }
    pub fn await_playback(mut self) -> Result<Self> {
        let found = {
            let mut await_event = |event| {
                self.events()
                    .find_map(|ev| {
                        ev.map(|ev| (ev == event).then_some(()))
                            .map_err(Error::WaitingForPlaybackStart)
                            .transpose()
                    })
                    .ok_or(Error::StreamClosed)
                    .and_then(identity)
            };
            Ok(())
                .and_then(|_| await_event(MpvEvent::File(FileEvent::FileLoaded)))
                .and_then(|_| await_event(MpvEvent::PlaybackControl(PlaybackControlEvent::PlaybackRestart)))
        };
        found.map(move |_| self)
    }

    pub fn finish(self) -> Result<()> {
        self.process
            .child
            .wait()
            .map_err(Error::WaitingForFinish)
            .and_then(|code| if code.success() { Ok(()) } else { Err(Error::BadStatusCode { code }) })
    }
}
