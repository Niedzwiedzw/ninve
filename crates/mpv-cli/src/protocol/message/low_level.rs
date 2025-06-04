use {
    property::{PropertyValueName, SetPropertyValue},
    serde::Serialize,
    std::sync::atomic::AtomicU64,
};

pub mod property;

#[derive(Debug, serde::Deserialize)]
pub struct SetProperty<T: SetPropertyValue>(pub T);

impl<T> Serialize for SetProperty<T>
where
    T: SetPropertyValue,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::json!(
            [serde_json::Value::String("set_property".into())]
                .into_iter()
                .chain(self.0.to_json_array())
                .collect::<Vec<_>>()
        )
        .serialize(serializer)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct GetProperty<T: PropertyValueName>(pub T);

impl<T: PropertyValueName> Serialize for GetProperty<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::json!(
            [serde_json::Value::String("get_property".into())]
                .into_iter()
                .chain(vec![serde_json::json!(T::as_str())])
                .collect::<Vec<_>>()
        )
        .serialize(serializer)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ObserveProperty<T: PropertyValueName>(pub T);

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

impl<T: PropertyValueName> Serialize for ObserveProperty<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::json!(
            [serde_json::Value::String("observe_property".into())]
                .into_iter()
                .chain(vec![
                    serde_json::json!(ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)),
                    serde_json::json!(T::as_str())
                ])
                .collect::<Vec<_>>()
        )
        .serialize(serializer)
    }
}
