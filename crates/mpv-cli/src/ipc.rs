use {
    crate::{
        instance::MpvInstance,
        protocol::{
            MpvCommand,
            event::MpvEvent,
            message::{ErrorResponse, IpcResponse, SuccessResponse},
        },
    },
    serde::{Serialize, de::DeserializeOwned},
    serde_json::{Map, Value},
    std::{
        any::type_name,
        convert::identity,
        fmt::Debug,
        io::{BufRead, Write},
    },
    tap::{Pipe, Tap},
    tracing::{debug, error, info, instrument, warn},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not serialize a command")]
    SerializingCommand {
        ty: &'static str,
        #[source]
        source: serde_json::Error,
    },
    #[error("Could not deserialize a command response:\n`{raw}`")]
    DeserializingResponse {
        ty: &'static str,
        raw: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("Could not write to stdin")]
    WritingToStdin(#[source] std::io::Error),
    #[error("Could not read from stdout")]
    ReadingFromStdout(#[source] std::io::Error),
    #[error("Running command\n{command}")]
    RunningCommand {
        command: String,
        #[source]
        source: Box<Self>,
    },
    #[error("Error came from ipc:\n{error:#?}")]
    IpcErrorsOccurred { error: ErrorResponse },
    #[error("Response did not arrive from protocol")]
    ResponseDidNotArrive,
    #[error("Response was not expected")]
    UnexpectedResponse,
    #[error("While handling response")]
    HandleResponse(#[source] ErrorResponse),
}

type Result<T> = std::result::Result<T, Error>;

pub enum HandledIpcResponse<T> {
    Success(SuccessResponse<T>),
    Event(MpvEvent),
}

impl<T> IpcResponse<T> {
    pub fn handle(self) -> std::result::Result<HandledIpcResponse<T>, ErrorResponse> {
        match self {
            IpcResponse::Error(error_response) => Err(error_response),
            IpcResponse::Event(event) => Ok(HandledIpcResponse::Event(event)),
            IpcResponse::Success(success_response) => Ok(HandledIpcResponse::Success(success_response)),
        }
    }
}

impl MpvInstance {
    #[instrument(skip(self), level = "DEBUG", ret, err)]
    pub fn command<C>(&mut self, command: C) -> Result<C::Response>
    where
        C: MpvCommand + Debug,
    {
        debug!("running command {command:?}");
        let command_debug = format!("{command:#?}");
        #[allow(deprecated)]
        self.command_raw::<C, C::Response>(command)
            .map_err(|source| Error::RunningCommand {
                command: command_debug,
                source: Box::new(source),
            })
    }
    fn with_next_line<T, F>(&mut self, with_next_line: F) -> std::io::Result<T>
    where
        F: FnOnce(&str) -> T,
    {
        self.line_buffer.clear();
        self.process
            .ipc_stream
            .read_line(&mut self.line_buffer)
            .map(|_| with_next_line(&self.line_buffer))
    }
    fn next_message_raw<R: DeserializeOwned + Debug>(&mut self) -> Result<IpcResponse<R>> {
        self.with_next_line(|line| {
            IpcResponse::<R>::from_json(line).map_err(|source| Error::DeserializingResponse {
                ty: type_name::<R>(),
                raw: line.to_string(),
                source,
            })
        })
        .map_err(Error::ReadingFromStdout)
        .and_then(identity)
    }

    pub fn next_event(&mut self) -> Result<MpvEvent> {
        self.next_message_raw::<()>().and_then(|message| {
            message
                .handle()
                .map_err(Error::HandleResponse)
                .and_then(|response| match response {
                    HandledIpcResponse::Success(success_response) => {
                        error!("unexpected response: {success_response:?}");
                        Err(Error::UnexpectedResponse)
                    }
                    HandledIpcResponse::Event(mpv_event) => Ok(mpv_event),
                })
        })
    }

    pub fn events(&mut self) -> impl Iterator<Item = Result<MpvEvent>> + '_ {
        std::iter::from_fn(|| Some(self.next_event()))
    }

    #[deprecated = "prefer .command()"]
    #[instrument(level = "TRACE", ret, err)]
    pub fn command_raw<C: Serialize + Debug, R: DeserializeOwned + Debug>(&mut self, command: C) -> Result<R> {
        debug!("running command {command:?}");
        serde_json::to_string(&command)
            .map_err(|source| Error::SerializingCommand { ty: type_name::<C>(), source })
            .and_then(|command| {
                debug!(" -> [ command ] {command}");
                self.process
                    .ipc_stream
                    .get_mut()
                    .pipe(|out| {
                        Ok(())
                            .and_then(|_| out.write_all(command.as_bytes()))
                            .and_then(|_| out.write("\n".as_bytes()))
                    })
                    .map_err(Error::WritingToStdin)
            })
            .and_then(|_| {
                loop {
                    match self
                        .next_message_raw::<R>()
                        .and_then(|message| message.handle().map_err(Error::HandleResponse))
                    {
                        Ok(response) => match response {
                            HandledIpcResponse::Success(success_response) => return Ok(success_response.data),
                            HandledIpcResponse::Event(mpv_event) => {
                                info!("[event] {mpv_event:?}");
                                self.events.push(mpv_event);
                                continue;
                            }
                        },
                        Err(error) => return Err(error),
                    }
                }
            })
    }
}
