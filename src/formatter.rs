use ansi_term::Color::*;
use ansi_term::Style;
use atty::Stream;
use chrono::{DateTime, Local};
use regex::{Captures, Regex};
use unicode_width::UnicodeWidthStr;

use bibi_types::{LiveStream, LiveStreamStatus};

fn pad_string_right(string: &str, max_width: &usize) -> String {
    string.to_owned() + &" ".repeat(max_width - string.width())
}

pub struct Formatter {
    format: String,
    regex: Regex,
}

impl Formatter {
    pub fn new(template: impl Into<String>) -> Self {
        let regex = Regex::new(r"\{([^{}]*)\}").unwrap();

        Formatter {
            format: template.into(),
            regex,
        }
    }

    pub fn print(self, lives: &[LiveStream]) {
        // TODO: Make these width variables more dynamic?
        // Get the width of the widest channel name
        let max_name_width = lives
            .iter()
            .fold(0, |acc, live| std::cmp::max(live.author_name.width(), acc));

        let max_handle_width = lives.iter().fold(0, |acc, live| {
            std::cmp::max(live.author_handle.width(), acc)
        });

        let max_title_width = lives
            .iter()
            .fold(0, |acc, live| std::cmp::max(live.title.width(), acc));

        let use_color = atty::is(Stream::Stdout);

        // TODO: Make this prettier
        let field_count = self.regex.find_iter(&self.format).count();

        for live in lives.iter() {
            let mut current_field_num = 0;

            let formatted = self
                .regex
                .replace_all(&self.format, |cap: &Captures| {
                    let captured = cap.get(1).unwrap().as_str();
                    current_field_num += 1;
                    let is_last_field = field_count <= current_field_num;

                    match captured {
                        "author_handle" => {
                            if is_last_field {
                                return live.author_handle.to_owned();
                            }
                            pad_string_right(&live.author_handle, &max_handle_width)
                        }
                        "author_name" => {
                            if is_last_field {
                                return live.author_name.to_owned();
                            }
                            pad_string_right(&live.author_name, &max_name_width)
                        }
                        "stream_time" => {
                            let local_time: DateTime<Local> = DateTime::from(live.time);
                            local_time.format("%H:%M").to_string()
                        }
                        "stream_id" => live.id.to_owned(),
                        "stream_url" => "https://youtu.be/".to_owned() + &live.id,
                        "stream_title" => {
                            if is_last_field {
                                return live.title.to_owned();
                            }
                            pad_string_right(&live.title, &max_title_width)
                        }
                        _ => cap.get(0).unwrap().as_str().to_owned(),
                    }
                })
                .to_string();

            let style = if use_color {
                match live.status {
                    LiveStreamStatus::Ended => Fixed(8).normal(),
                    LiveStreamStatus::Live => Purple.normal(),
                    LiveStreamStatus::Upcoming => Style::default(),
                }
            } else {
                Style::default()
            };

            println!("{}", style.paint(formatted));
        }
    }
}
