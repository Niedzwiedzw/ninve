use {property::PropertyValue, serde::Serialize};

pub mod property;

#[derive(Debug)]
pub struct SetProperty<T: PropertyValue>(pub T);

impl<T> Serialize for SetProperty<T>
where
    T: PropertyValue,
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
