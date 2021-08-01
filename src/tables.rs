use crate::files::Config;

use chrono::Duration;
use chrono::MIN_DATETIME;
use chrono::NaiveDate;

pub fn create_hours(config_file: &Config) -> String {
    let day_hours = MIN_DATETIME + Duration::hours(24);
    let mut day = String::new();
    let mut looptime = MIN_DATETIME;

    let min_line_length = config_file.hours.min_line_length as usize;
    
    let hours_div = {
        if config_file.hours.hours > 24 {
            24
        } else {
           config_file.hours.hours 
        }
    };
    
    let text_format = &config_file.hours.text_format;

    while looptime < day_hours {
        day = day + &looptime.format(text_format).to_string();
        day.push_str(&config_file.hours.horizontal_divisor.repeat(min_line_length-text_format.len()));
        day.push_str("\n");
        day.push_str(&config_file.hours.vertical_divisor);
        day.push_str(&" ".repeat(min_line_length - 2));
        day.push_str(&config_file.hours.vertical_divisor);
        day.push_str("\n");

        looptime = looptime + Duration::hours(24)/(24/hours_div);
    }

    day.push_str(&"-".repeat(min_line_length));
    day
}

pub fn create_week(config_file: &Config) -> String {
    let mut weekdays = vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    if config_file.week.starts_on_monday {
        weekdays = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    }
    let mut week = String::new();
    let mut week_as_vec: Vec<String> = Vec::new();

    let mut max_lines: u32 = 0;

    for weekday in weekdays.iter() {
        let day = create_hours(&config_file);
        let day_as_vec = day.split("\n").collect::<Vec<&str>>();

        if day_as_vec.len() as u32 > max_lines {
            max_lines = day_as_vec.len() as u32;
        }
        week_as_vec.push(day.to_string());

        week.push_str(weekday);
        week.push_str(&"-".repeat(&day_as_vec[0].len() - weekday.len()));
    }
    week.push_str("\n");

    for i in 0..max_lines {
        for day in week_as_vec.iter() {
            week.push_str(
                {
                    let x = day.split("\n").collect::<Vec<&str>>();
                    x[i as usize]
                }
            );
        }
        week.push_str("\n");
    }
    week
}