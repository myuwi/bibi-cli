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
    pub handle: Option<&'static str>,
    pub name: &'static str,
    pub branch: Branch,
}

#[derive(Clone, Debug)]
pub enum LiveStreamStatus {
    Ended,
    Live,
    Upcoming,
}

#[derive(Clone, Debug)]
pub struct HoloduleData {
    pub id: String,
    pub time: DateTime<FixedOffset>,
    pub status: LiveStreamStatus,
}

pub struct LiveStream {
    pub id: String,
    pub title: String,
    pub author_name: String,
    pub author_id: String,
    pub status: LiveStreamStatus,
    pub time: DateTime<FixedOffset>,
}

#[derive(Deserialize, Default, Debug)]
pub struct OEmbedData {
    pub title: String,
    pub author_name: String,
    pub author_url: String,
}
