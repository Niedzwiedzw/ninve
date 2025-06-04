use super::*;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "event")]
pub enum FileEvent {
    StartFile(StartFileEvent),
    EndFile(EndFileEvent),
    FileLoaded,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "event")]
pub enum PlaybackControlEvent {
    Seek,
    PlaybackRestart,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "event")]
pub enum SystemEvent {
    Shutdown,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "event")]
pub enum ReplyEvent {
    GetPropertyReply(GetPropertyReplyEvent),
    SetPropertyReply(SetPropertyReplyEvent),
    CommandReply(CommandReplyEvent),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "event")]
pub enum ConfigEvent {
    VideoReconfig,
    AudioReconfig,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "event")]
pub enum MessageEvent {
    LogMessage(LogMessageEvent),
    Hook(HookEvent),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "event")]
pub enum ClientInteractionEvent {
    ClientMessage(ClientMessageEvent),
    PropertyChange(PropertyChangeEvent),
}
