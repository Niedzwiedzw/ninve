use {
    super::message::low_level::property::{self, PropertyValueName},
    serde::{Deserialize, Serialize},
};

pub mod grouped_events;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("found value of type `{ty}`, but could not parse for {property_name:?}")]
    BadType {
        ty: &'static str,
        property_name: property::PropertyValueKind,
        #[source]
        source: serde_json::Error,
    },
}
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, enum_kinds::EnumKind)]
#[enum_kind(MpvEventKind, derive(Serialize, strum::Display, PartialOrd, Ord, Hash))]
#[serde(untagged)]
pub enum MpvEvent {
    File(grouped_events::FileEvent),
    PlaybackControl(grouped_events::PlaybackControlEvent),
    System(grouped_events::SystemEvent),
    Reply(grouped_events::ReplyEvent),
    Config(grouped_events::ConfigEvent),
    Message(grouped_events::MessageEvent),
    ClientInteraction(grouped_events::ClientInteractionEvent),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct BaseEvent {
    #[serde(default)]
    pub id: Option<u64>, // reply_userdata, optional as it may be 0 (not added)
    #[serde(default)]
    pub error: Option<String>, // Optional, missing if no error
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StartFileEvent {
    #[serde(flatten)]
    pub base: BaseEvent,
    pub playlist_entry_id: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EndFileReason {
    Eof,
    Stop,
    Quit,
    Error,
    Redirect,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct EndFileEvent {
    #[serde(flatten)]
    pub base: BaseEvent,
    pub reason: EndFileReason,
    pub playlist_entry_id: u64,
    #[serde(default)]
    pub file_error: Option<String>, // Optional, unset if no error known
    #[serde(default)]
    pub playlist_insert_id: Option<u64>, // Only present if redirect
    #[serde(default)]
    pub playlist_insert_num_entries: Option<u64>, // Only present if playlist_insert_id is present
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct LogMessageEvent {
    #[serde(flatten)]
    pub base: BaseEvent,
    pub prefix: String,
    pub level: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct HookEvent {
    #[serde(flatten)]
    pub base: BaseEvent,
    pub hook_id: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct GetPropertyReplyEvent {
    #[serde(flatten)]
    pub base: BaseEvent,
    pub result: Option<serde_json::Value>, // mpv_node type, represented as JSON value
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct SetPropertyReplyEvent {
    #[serde(flatten)]
    pub base: BaseEvent,
    pub result: Option<serde_json::Value>, // mpv_node type, represented as JSON value
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CommandReplyEvent {
    #[serde(flatten)]
    pub base: BaseEvent,
    pub result: Option<serde_json::Value>, // mpv_node type, represented as JSON value
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ClientMessageEvent {
    #[serde(flatten)]
    pub base: BaseEvent,
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PropertyChangeEvent {
    #[serde(flatten)]
    pub base: BaseEvent,
    pub name: property::PropertyValueKind,
    pub data: Option<serde_json::Value>, // mpv_node type, represented as JSON value
}

impl PropertyChangeEvent {
    pub fn of_kind<K: PropertyValueName>(&self) -> Option<Result<K::Value>> {
        (K::as_str() == self.name.name())
            .then_some(())
            .and_then(|_| self.data.clone())
            .map(|data| {
                serde_json::from_value::<K::Value>(data).map_err(|source| self::Error::BadType {
                    ty: std::any::type_name::<K::Value>(),
                    property_name: self.name,
                    source,
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use {super::*, serde_json::from_str};
    type Result<T> = std::result::Result<T, serde_json::Error>;

    fn parse_event(ev: &str) -> Result<()> {
        from_str::<MpvEvent>(ev).map(|_| ())
    }
    #[test]
    fn test_example_1() -> Result<()> {
        parse_event(r#"{"event":"start-file","playlist_entry_id":1}"#)
    }
}
