use {
    super::HandledIpcResponse,
    crate::{
        protocol::{
            event::MpvEvent,
            message::{ErrorResponse, IpcResponse},
        },
        utils::abort_on_drop::{AbortOnDrop, AbortOnDropExt},
    },
    futures::{FutureExt, TryFutureExt},
    serde::de::DeserializeOwned,
    serde_json::Value,
    split::{BusCommandsHalf, BusEventsHalf},
    std::{any::type_name, fmt::Debug, future::ready, sync::Arc},
    tap::{Pipe, Tap, TapFallible},
    tokio::{
        io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
        net::{
            UnixStream,
            unix::{OwnedReadHalf, OwnedWriteHalf},
        },
        sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    },
    tokio_stream::wrappers::UnboundedReceiverStream,
    tracing::{debug, instrument},
};

#[derive(Debug)]
#[allow(dead_code)]
struct LogDrop<T: Debug>(pub T, &'static str);

impl<T: Debug> Drop for LogDrop<T> {
    fn drop(&mut self) {
        debug!(kind=%type_name::<T>(), "DROPPING VALUE: {self:?}");
    }
}

#[extension_traits::extension(trait LogDropExt)]
impl<T: Debug> T
where
    Self: Sized,
{
    fn log_drop(self, message: &'static str) -> LogDrop<T> {
        LogDrop(self, message)
    }
}

#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum MaybeDeserializedResponse {
    Raw(String),
    Value(Value),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("While handling response")]
    HandleResponse(#[source] ErrorResponse),
    #[error("Could not read from stdout")]
    ReadingFromStdout(#[source] std::io::Error),
    #[error("Could not deserialize a command response:\n`{raw}`")]
    DeserializingResponse {
        ty: &'static str,
        raw: MaybeDeserializedResponse,
        #[source]
        source: serde_json::Error,
    },
    #[error("Could not write to stdin")]
    WritingToStdin(#[source] std::io::Error),
    #[error("Sending to responses")]
    SendingToResponses,
    #[error("Sending to events")]
    SendingToEvents,
    // #[error("Underlying unix stream returned Ok(0) (EOF)")]
    // UnixStreamEof,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct EventBus {
    pub commands: BusCommandsHalf,
    pub events: BusEventsHalf,
}

struct EventBusWriteHandler {
    command: UnboundedReceiver<String>,
    write_half: LogDrop<OwnedWriteHalf>,
}

impl EventBusWriteHandler {
    async fn run(self) -> Result<()> {
        self.pipe(async |Self { mut command, mut write_half }| -> Result<()> {
            while let Some(next) = command.recv().await {
                write_half
                    .0
                    .write_all(next.as_bytes())
                    .await
                    .map_err(Error::WritingToStdin)?;
                write_half
                    .0
                    .write_all("\n".as_bytes())
                    .await
                    .map_err(Error::WritingToStdin)?;
                write_half.0.flush().await.map_err(Error::WritingToStdin)?;
            }
            tracing::warn!("writer died");
            Ok(())
        })
        .await
    }
}

#[derive(Debug)]
struct EventBusTaskGuard {
    #[allow(dead_code)]
    rx_join_handle: AbortOnDrop<()>,
    #[allow(dead_code)]
    tx_join_handle: AbortOnDrop<()>,
}

struct EventBusReadHandler {
    responses: UnboundedSender<serde_json::Value>,
    events: UnboundedSender<MpvEvent>,
    line_buffer: String,
    read_half: LogDrop<BufReader<OwnedReadHalf>>,
}
impl EventBusReadHandler {
    async fn with_next_line<T, F>(&mut self, with_next_line: F) -> std::io::Result<T>
    where
        F: AsyncFnOnce(&str) -> T,
    {
        self.line_buffer.clear();
        self.read_half
            .0
            .read_line(&mut self.line_buffer)
            .await
            .pipe(ready)
            .and_then(|_| with_next_line(&self.line_buffer).map(Ok))
            .await
    }

    #[instrument(skip(self))]
    async fn next_message_raw<R: DeserializeOwned + Debug>(&mut self) -> Result<Option<IpcResponse<R>>> {
        self.with_next_line(async |line| {
            if line.is_empty() {
                Ok(None)
            } else {
                IpcResponse::<R>::from_json(line)
                    .map_err(|source| Error::DeserializingResponse {
                        ty: type_name::<R>(),
                        raw: line.to_string().into(),
                        source,
                    })
                    .map(Some)
            }
        })
        .map_err(Error::ReadingFromStdout)
        .and_then(ready)
        .await
    }
    #[instrument(skip(self))]
    pub async fn run(mut self) -> Result<()> {
        loop {
            match self
                .next_message_raw::<serde_json::Value>()
                .and_then(|message| {
                    message
                        .map(|m| m.handle().map_err(Error::HandleResponse))
                        .transpose()
                        .pipe(ready)
                })
                .await?
                .tap(|res| {
                    if let Some(res) = res.as_ref() {
                        tracing::debug!("[MESSAGE] {res:?}")
                    }
                }) {
                Some(HandledIpcResponse::Success(success)) => self
                    .responses
                    .send(success.tap(|success| debug!("[SUCCESS] {success}")))
                    .map_err(|_| Error::SendingToResponses)
                    .tap_err(|e| tracing::error!("{e:?}"))?,
                Some(HandledIpcResponse::Event(mpv_event)) => self
                    .events
                    .send(mpv_event.tap(|event| debug!("[EVENT] {event:?}")))
                    .map_err(|_| Error::SendingToEvents)
                    .tap_err(|e| tracing::error!("{e:?}"))?,
                None => {}
            }
        }
    }
}

pub mod split {
    use {super::EventBusTaskGuard, crate::protocol::event::MpvEvent, std::sync::Arc, tokio::sync::mpsc, tokio_stream::wrappers::UnboundedReceiverStream};

    #[derive(Debug)]
    pub struct BusEventsHalf {
        #[allow(dead_code)]
        pub(super) task_guard: Arc<EventBusTaskGuard>,
        pub events: UnboundedReceiverStream<MpvEvent>,
    }

    #[derive(Debug)]
    pub struct BusCommandsHalf {
        #[allow(dead_code)]
        pub(super) task_guard: Arc<EventBusTaskGuard>,
        pub responses: mpsc::UnboundedReceiver<serde_json::Value>,
        pub commands: mpsc::UnboundedSender<String>,
    }
}

impl EventBus {
    pub fn spawn(stream: UnixStream) -> Self {
        debug!("spawning event bus");
        let (read_half, write_half) = stream.into_split();
        let (responses_tx, responses_rx) = mpsc::unbounded_channel();
        let (events_tx, events_rx) = mpsc::unbounded_channel();
        let (commands_tx, commands_rx) = mpsc::unbounded_channel();

        let rx_join_handle = tokio::task::spawn(async move {
            EventBusReadHandler {
                responses: responses_tx,
                events: events_tx,
                line_buffer: Default::default(),
                read_half: BufReader::new(read_half).log_drop("buf reader of read half"),
            }
            .run()
            .await
            .expect("read event bus crashed")
        });
        let tx_join_handle = tokio::task::spawn(async move {
            EventBusWriteHandler {
                command: commands_rx,
                write_half: write_half.log_drop("write half"),
            }
            .run()
            .await
            .expect("write event bus crashed")
        });
        let task_guard = Arc::new(EventBusTaskGuard {
            rx_join_handle: rx_join_handle.abort_on_drop(),
            tx_join_handle: tx_join_handle.abort_on_drop(),
        });
        Self {
            commands: BusCommandsHalf {
                task_guard: task_guard.clone(),
                responses: responses_rx,
                commands: commands_tx,
            },
            events: BusEventsHalf {
                task_guard: task_guard.clone(),
                events: events_rx.pipe(UnboundedReceiverStream::new),
            },
        }
    }
}
