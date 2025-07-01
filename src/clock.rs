use crate::config::ClockConfig;
use std::thread;
use std::time::Duration;
use time::{OffsetDateTime, UtcDateTime, UtcOffset, macros::format_description};
pub struct DotClock<'a> {
    config: &'a ClockConfig,
    now: OffsetDateTime,
}

impl<'a> DotClock<'a> {
    pub fn new(config: &'a ClockConfig) -> Self {
        let now = match Self::get_current_time(config) {
            Ok(now) => now,
            Err(e) => {
                eprintln!("Error obtaining current time: {}", e);
                std::process::exit(1);
            }
        };
        Self { config, now }
    }

    fn get_current_time(config: &ClockConfig) -> Result<OffsetDateTime, String> {
        let maybe_offset = Self::parse_offset(config.offset.as_deref());
        match maybe_offset {
            Some(offset) => Ok(UtcDateTime::now().to_offset(offset)),
            None => Ok(OffsetDateTime::now_utc()),
        }
    }

    pub fn display(&self) {
        for field in &self.config.format.order {
            match field.as_str() {
                "year" if self.config.show_date => {
                    let (tens, units) = Self::year_2digit(self.now.year());
                    Self::print_to_dots(tens);
                    Self::print_to_dots(units);
                }
                "month" if self.config.show_date => Self::print_to_dots(self.now.month() as u32),
                "day" if self.config.show_date => Self::print_to_dots(self.now.day() as u32),
                "hour" if self.config.show_time => Self::print_to_dots(self.now.hour() as u32),
                "minute" if self.config.show_time => Self::print_to_dots(self.now.minute() as u32),
                "second" if self.config.show_time => Self::print_to_dots(self.now.second() as u32),
                _ => print!("{}", field),
            }
        }
        println!();
    }

    fn parse_offset(offset_str: Option<&str>) -> Option<UtcOffset> {
        let offset_str = offset_str?;
        let formats = [
            format_description!("[offset_hour][offset_minute]"),
            format_description!("[offset_hour]:[offset_minute]"),
            format_description!("[offset_hour]"),
        ];
        formats
            .iter()
            .find_map(|fmt| UtcOffset::parse(offset_str, fmt).ok())
    }

    fn print_to_dots(value: u32) {
        let dot = Self::number_to_dots(value).unwrap_or('?');
        print!("{}", dot);
    }

    fn number_to_dots(number: u32) -> Option<char> {
        if number <= 0xFF {
            char::from_u32(0x2800 + number)
        } else {
            None
        }
    }

    fn year_2digit(full_year: i32) -> (u32, u32) {
        let y = (full_year % 100) as u32;
        (y / 10, y % 10)
    }

    pub fn run_loop(&self) {
        loop {
            let refreshed = match DotClock::get_current_time(self.config) {
                Ok(now) => DotClock {
                    config: self.config,
                    now,
                },
                Err(e) => {
                    eprintln!("Error refreshing time: {}", e);
                    std::process::exit(1);
                }
            };
            refreshed.display();
            thread::sleep(Duration::from_secs(1));
        }
    }
}
