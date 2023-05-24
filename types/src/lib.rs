use chrono::{offset::FixedOffset, DateTime};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Branch {
    Hololive,
    Holostars,
}

#[derive(Deserialize, Debug)]
pub struct Channel {
    pub id: &'static str,
    pub handle: &'static str,
    pub name: &'static str,
    pub branch: Branch,
}

#[derive(Clone, Debug)]
pub enum LiveStreamStatus {
    Ended,
    Live,
    Upcoming,
}

#[derive(Debug)]
pub struct HoloduleData {
    pub id: String,
    pub time: DateTime<FixedOffset>,
    pub status: LiveStreamStatus,
}

pub struct LiveStream {
    pub channel: Option<&'static Channel>,
    pub author_name: String,
    pub author_handle: String,
    pub id: String,
    pub title: String,
    pub status: LiveStreamStatus,
    pub time: DateTime<FixedOffset>,
}

impl LiveStream {
    // TODO: Add support for channel id and channel handle without @ symbol
    pub fn is_from_author(&self, handle_or_id: &str) -> bool {
        let handle_or_id_cleaned = handle_or_id.replace('@', "");

        if let Some(c) = self.channel {
            c.handle.replace('@', "").to_lowercase() == handle_or_id_cleaned.to_lowercase()
                || c.id == handle_or_id_cleaned
        } else {
            self.author_handle.replace('@', "").to_lowercase()
                == handle_or_id_cleaned.to_lowercase()
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct OEmbedData {
    pub title: String,
    pub author_name: String,
    pub author_url: String,
}
