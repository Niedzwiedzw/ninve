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
pub struct SuccessResponse<T> {
    #[allow(dead_code)]
    error: type_guard::SuccessGuard,
    pub request_id: i64,
    pub data: T,
}

#[derive(Debug, Deserialize, Clone, thiserror::Error)]
#[error("Error response: {error}")]
pub struct ErrorResponse {
    pub error: Box<str>,
}

#[derive(Debug, Clone)]
pub enum IpcResponse<T> {
    Success(SuccessResponse<T>),
    Error(ErrorResponse),
    Event(MpvEvent),
}

impl<T: DeserializeOwned> IpcResponse<T> {
    pub fn from_json(json: &str) -> std::result::Result<Self, serde_json::Error> {
        from_str::<Value>(json).and_then(|value| match &value {
            Value::Object(map) => match map.get("error") {
                Some(Value::String(s)) if s.as_str() == "success" => from_value::<SuccessResponse<_>>(value).map(Self::Success),
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

pub mod api {
    use {
        super::low_level::{SetProperty, property::PropertyValue},
        crate::protocol::MpvCommand,
        serde::Serialize,
        std::fmt::Debug,
    };
    pub mod common_responses {
        pub type Empty = ();
    }

    #[derive(derive_more::Constructor, Debug)]
    pub struct SetPropertyCommand<T>
    where
        T: PropertyValue,
    {
        pub command: SetProperty<T>,
    }

    impl<T> Serialize for SetPropertyCommand<T>
    where
        T: PropertyValue,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serde_json::json!({"command": self.command }).serialize(serializer)
        }
    }

    impl<T> MpvCommand for SetPropertyCommand<T>
    where
        T: PropertyValue + Debug,
    {
        type Response = common_responses::Empty;
    }
}

pub mod low_level;
