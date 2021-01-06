use snafu::{Snafu, ResultExt};

mod root;
pub mod actions;
pub mod participants;
pub mod generic_types;
pub mod header;

/// An error returned when deserializing data from YouTube
#[derive(Debug, Snafu)]
pub enum YouTubeDeserializeError {
    #[snafu(display("Couldn't extract data from json. Reason: {},\njson: {}", source, json))]
    DeserializeJson {
        json: String,
        source: serde_json::Error
    },
}

pub type Result<T> = std::result::Result<T, YouTubeDeserializeError>;
pub type InitialChatJson = root::InitialChatJson;
pub type ChatJson = root::ChatJson;
pub type ParamsContext = root::ParamsContext;
pub type YoutubeParams = root::YoutubeParams;
pub type AdSignalsInfo = root::AdSignalsInfo;

pub struct Youtube;

impl Youtube {
    pub fn deserialize_initial(json: &str) -> Result<InitialChatJson> {
        serde_json::from_str::<InitialChatJson>(json)
            .context(DeserializeJson { json: json.to_string() })
    }

    pub fn deserialize(json: &str) -> Result<ChatJson> {
        serde_json::from_str::<ChatJson>(json)
            .context(DeserializeJson { json: json.to_string() })
    }
}