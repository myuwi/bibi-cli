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

#[derive(Deserialize, Debug)]
pub struct OEmbedData {
    pub title: String,
    pub author_name: String,
    pub author_url: String,
}

pub enum Branch {
    Hololive,
    Holostars,
}

pub struct LiveStream {
    pub author_name: String,
    pub author_handle: String,
    pub id: String,
    pub title: String,
    pub status: LiveStreamStatus,
    pub time: DateTime<FixedOffset>,
}

impl LiveStream {
    // Probably not 100% accurate, but works relatively well for now
    pub fn get_branch(&self) -> Branch {
        let author_name = self.author_name.to_lowercase();

        if self.title.contains("ホロスタ")
            || self.title.contains("ホロスターズ")
            || author_name.contains("holostars")
            || author_name.contains("uproar")
        {
            return Branch::Holostars;
        }

        Branch::Hololive
    }
}
