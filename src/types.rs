use chrono::{offset::FixedOffset, DateTime};
use serde::Deserialize;

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
    pub author_name: String,
    pub author_handle: String,
    pub id: String,
    pub title: String,
    pub status: LiveStreamStatus,
    pub time: DateTime<FixedOffset>,
}

#[derive(Deserialize, Debug)]
pub struct OEmbedData {
    pub title: String,
    pub author_name: String,
    pub author_url: String,
}
