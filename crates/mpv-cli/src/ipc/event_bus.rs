use {
    super::{Error, HandledIpcResponse, Result},
    crate::{
        protocol::{MpvCommand, event::MpvEvent, message::IpcResponse},
        utils::abort_on_drop::{AbortOnDrop, AbortOnDropExt},
    },
    futures::{FutureExt, TryFutureExt},
    serde::de::DeserializeOwned,
    std::{any::type_name, future::ready, io::Read},
    tap::Pipe,
    tokio::{
        io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
        net::{
            UnixStream,
            unix::{OwnedReadHalf, OwnedWriteHalf},
        },
        sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
        task::JoinHandle,
    },
};

pub(crate) struct EventBus {
    responses: UnboundedReceiver<serde_json::Value>,
    events: UnboundedReceiver<MpvEvent>,
    commands: UnboundedSender<String>,
    task_guard: EventBusTaskGuard,
}

struct EventBusReadHandler {
    responses: UnboundedSender<serde_json::Value>,
    events: UnboundedSender<MpvEvent>,
    line_buffer: String,
    read_half: BufReader<OwnedReadHalf>,
}

struct EventBusWriteHandler {
    command: UnboundedReceiver<String>,
    write_half: OwnedWriteHalf,
}

impl EventBusWriteHandler {
    async fn run(self) -> Result<()> {
        self.pipe(async |Self { mut command, mut write_half }| -> Result<()> {
            while let Some(next) = command.recv().await {
                write_half
                    .write(next.as_bytes())
                    .await
                    .map_err(Error::WritingToStdin)?;
                write_half
                    .write("\n".as_bytes())
                    .await
                    .map_err(Error::WritingToStdin)?;
            }
            Ok(())
        })
        .pipe(|_| Ok(()))
    }
}

struct EventBusTaskGuard {
    rx_join_handle: AbortOnDrop<Result<()>>,
    tx_join_handle: AbortOnDrop<Result<()>>,
}

impl EventBusReadHandler {
    async fn with_next_line<T, F>(&mut self, with_next_line: F) -> std::io::Result<T>
    where
        F: AsyncFnOnce(&str) -> T,
    {
        self.line_buffer.clear();
        self.read_half
            .read_line(&mut self.line_buffer)
            .await
            .pipe(ready)
            .and_then(|_| with_next_line(&self.line_buffer).map(Ok))
            .await
    }
    async fn next_message_raw<R: DeserializeOwned + std::fmt::Debug>(&mut self) -> Result<IpcResponse<R>> {
        self.with_next_line(async |line| {
            IpcResponse::<R>::from_json(line).map_err(|source| Error::DeserializingResponse {
                ty: type_name::<R>(),
                raw: line.to_string(),
                source,
            })
        })
        .map_err(Error::ReadingFromStdout)
        .and_then(ready)
        .await
    }

    pub async fn run(mut self) -> Result<()> {
        loop {
            match self
                .next_message_raw::<serde_json::Value>()
                .and_then(|message| message.handle().map_err(Error::HandleResponse).pipe(ready))
                .await?
            {
                HandledIpcResponse::Success(success) => self.responses.send(success).expect("sending response"),
                HandledIpcResponse::Event(mpv_event) => self.events.send(mpv_event).expect("sending event"),
            }
        }
    }
}

impl EventBus {
    pub fn spawn<R: Read>(stream: UnixStream) -> Self {
        let (read_half, write_half) = stream.into_split();
        let (responses_tx, responses_rx) = mpsc::unbounded_channel();
        let (events_tx, events_rx) = mpsc::unbounded_channel();
        let (commands_tx, commands_rx) = mpsc::unbounded_channel();

        let rx_join_handle = tokio::task::spawn(async move {
            EventBusReadHandler {
                responses: responses_tx,
                events: events_tx,
                line_buffer: Default::default(),
                read_half: BufReader::new(read_half),
            }
            .run()
            .await
        });
        let tx_join_handle = tokio::task::spawn(async move {
            EventBusWriteHandler {
                command: commands_rx,
                write_half,
            }
            .run()
            .await
        });
        Self {
            responses: responses_rx,
            events: events_rx,
            commands: commands_tx,
            task_guard: EventBusTaskGuard {
                rx_join_handle: rx_join_handle.abort_on_drop(),
                tx_join_handle: tx_join_handle.abort_on_drop(),
            },
        }
    }
}
