use {
    crate::{
        instance::MpvInstance,
        protocol::{
            MpvCommand,
            event::MpvEvent,
            message::{BaseResponse, ErrorResponse, IpcResponse},
        },
    },
    futures::TryFutureExt,
    serde::{Deserialize, Serialize, de::DeserializeOwned},
    serde_json::Value,
    std::{any::type_name, fmt::Debug, future::ready, process::ExitStatus},
    tap::Pipe,
    tokio::task::block_in_place,
    tracing::{debug, error, instrument},
};

#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum MaybeDeserializedResponse {
    Raw(String),
    Value(Value),
}

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
        raw: MaybeDeserializedResponse,
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
    #[error("While killing MPV")]
    KillingMpv(#[source] Box<crate::instance::Error>),
    #[error("Sending command to process (channel closed)")]
    SendingCommand,
    #[error("Reading response from command bus (channel closed)")]
    ReceivingResponseChannelClosed,
}

type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WithBaseResponse<T> {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(flatten)]
    pub value: T,
}

#[derive(Debug)]
pub enum HandledIpcResponse<T> {
    Success(T),
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

pub mod event_bus;

impl MpvInstance {
    pub async fn kill(self) -> Result<ExitStatus> {
        self.process
            .kill()
            .await
            .map_err(Box::new)
            .map_err(Error::KillingMpv)
    }
    #[instrument(skip(self), level = "DEBUG", ret, err)]
    pub async fn command<C>(&mut self, command: C) -> Result<C::Response>
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
            .await
    }

    async fn next_response_raw<R: DeserializeOwned + Debug>(&mut self) -> Result<R> {
        self.process
            .event_bus
            .responses
            .recv()
            .await
            .ok_or(Error::ReceivingResponseChannelClosed)
            .and_then(|response| {
                block_in_place(|| {
                    serde_json::from_value::<R>(response.clone()).map_err(|source| Error::DeserializingResponse {
                        ty: type_name::<R>(),
                        raw: response.clone().into(),
                        source,
                    })
                })
            })
    }
    #[deprecated = "prefer .command()"]
    #[instrument(level = "TRACE", ret, err)]
    pub async fn command_raw<C: Serialize + Debug, R: DeserializeOwned + Debug>(&mut self, command: C) -> Result<R> {
        debug!("running command {command:?}");
        block_in_place(|| serde_json::to_string(&command))
            .map_err(|source| Error::SerializingCommand { ty: type_name::<C>(), source })
            .and_then(|command| {
                debug!(" -> [ command ] {command}");
                self.process
                    .event_bus
                    .commands
                    .send(command)
                    .map_err(|_| Error::SendingCommand)
            })
            .pipe(ready)
            .and_then(|_| self.next_response_raw())
            .await
    }
}
