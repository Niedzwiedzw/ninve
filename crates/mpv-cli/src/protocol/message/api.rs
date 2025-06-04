use {
    super::{
        BaseResponse,
        WithBaseResponse,
        low_level::{
            GetProperty,
            ObserveProperty,
            SetProperty,
            property::{PropertyValueName, SetPropertyValue},
        },
    },
    crate::protocol::MpvCommand,
    serde::{Serialize, de::DeserializeOwned},
    std::fmt::Debug,
};

// observe_property
#[derive(derive_more::Constructor, Debug)]
pub struct ObservePropertyCommand<T>
where
    T: PropertyValueName,
{
    pub command: ObserveProperty<T>,
}

impl<T> Serialize for ObservePropertyCommand<T>
where
    T: PropertyValueName,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::json!({"command": self.command }).serialize(serializer)
    }
}

impl<T> MpvCommand for ObservePropertyCommand<T>
where
    T: PropertyValueName + Debug,
{
    type Response = BaseResponse;
}

// get_property
#[derive(derive_more::Constructor, Debug)]
pub struct GetPropertyCommand<T>
where
    T: PropertyValueName,
{
    pub command: GetProperty<T>,
}

impl<T> Serialize for GetPropertyCommand<T>
where
    T: PropertyValueName,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::json!({"command": self.command }).serialize(serializer)
    }
}

impl<T> MpvCommand for GetPropertyCommand<T>
where
    T: PropertyValueName + Debug,
    T::Value: DeserializeOwned + Debug,
{
    type Response = WithBaseResponse<T::Value>;
}

// set_property
#[derive(derive_more::Constructor, Debug)]
pub struct SetPropertyCommand<T>
where
    T: SetPropertyValue,
{
    pub command: SetProperty<T>,
}

impl<T> Serialize for SetPropertyCommand<T>
where
    T: SetPropertyValue,
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
    T: SetPropertyValue + Debug,
{
    type Response = BaseResponse;
}

#[cfg(test)]
pub mod tests {
    use {
        super::*,
        crate::protocol::{
            self,
            message::{
                IpcResponse,
                low_level::property::{self, PropertyValueName},
            },
        },
        tap::Tap,
    };

    #[test_log::test]
    fn test_example_response_1() {
        const RESPONSE: &str = r#"{"data":1055736,"error":"success","request_id":0}"#;
        IpcResponse::<serde_json::Value>::from_json(RESPONSE)
            .unwrap()
            .tap(|v| println!("{v:#?}"));
    }
}
