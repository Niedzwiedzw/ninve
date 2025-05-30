use {
    serde::{Deserialize, Deserializer, Serialize, Serializer, de},
    std::fmt,
};

macro_rules! serde_type_guard {
    ($name:ident, $identifier:literal) => {
        #[doc = concat!("Fails deserialization if value is not \"", $identifier, "\".")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $name;

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str($identifier)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value: String = Deserialize::deserialize(deserializer)?;
                if value == $identifier {
                    Ok($name)
                } else {
                    Err(de::Error::custom(format!("Expected \"{}\", but found \"{}\"", $identifier, value)))
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $identifier)
            }
        }
    };
}

serde_type_guard!(SuccessGuard, "success");
