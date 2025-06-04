use {
    super::event::MpvEvent,
    serde::{
        Deserialize,
        de::{DeserializeOwned, Error},
    },
    serde_json::{Value, from_str, from_value},
};

mod type_guard;

#[derive(Debug, Deserialize, Clone)]
pub struct BaseResponse {
    #[allow(dead_code)]
    error: type_guard::SuccessGuard,
    pub request_id: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WithBaseResponse<T> {
    #[allow(dead_code)]
    #[serde(flatten)]
    base: BaseResponse,
    pub data: T,
}

#[derive(Debug, Deserialize, Clone, thiserror::Error)]
#[error("Error response: {error}")]
pub struct ErrorResponse {
    pub error: Box<str>,
}

#[derive(Debug, Clone)]
pub enum IpcResponse<T> {
    Success(T),
    Error(ErrorResponse),
    Event(MpvEvent),
}

impl<T: DeserializeOwned> IpcResponse<T> {
    pub fn from_json(json: &str) -> std::result::Result<Self, serde_json::Error> {
        from_str::<Value>(json).and_then(|value| match &value {
            Value::Object(map) => match map.get("error") {
                Some(Value::String(s)) if s.as_str() == "success" => from_value::<_>(value).map(Self::Success),
                _ => Err(())
                    .or_else(|_| from_value::<MpvEvent>(value.clone()).map(Self::Event))
                    .or_else(|reason| {
                        from_value::<ErrorResponse>(value.clone())
                            .map(Self::Error)
                            .map_err(|e| serde_json::Error::custom(format!("{e:?}\ntried because: {reason:?}")))
                    }),
            },
            other => Err(serde_json::Error::custom(format!("expected map, found {other:?}"))),
        })
    }
}

pub mod api;

pub mod low_level;
