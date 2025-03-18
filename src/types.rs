use chrono::{offset::FixedOffset, DateTime};
use regex::Regex;
use serde::Deserialize;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

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

    pub fn get_name(&self) -> String {
        let re = Regex::new(r"\s*(C|(\sc))h(annel|\.|。|\s)\s*").unwrap();

        let parts: Vec<&str> = re.split(&self.author_name).collect();

        let name = match parts.as_slice() {
            [name, ""] => name.to_string(),
            [start, end] if !start.is_empty() => {
                if end.chars().all(|c| c.width().unwrap_or(1) == 1) {
                    start.to_string()
                } else {
                    end.split(' ')
                        .next()
                        .unwrap_or("")
                        .replace("チャンネル", "")
                }
            }
            _ => {
                let name = &self.author_name;

                if name.len() != name.width() {
                    name.split(' ').find(|part| part.len() != part.width())
                } else {
                    name.split(" hololive").next()
                }
                .unwrap_or(name)
                .to_string()
            }
        };

        name
    }
}
