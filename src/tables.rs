use crate::files::Config;
use crate::files;

use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::prelude::Local;
use chrono::Datelike;

pub fn create_hours(config_file: &Config, chosen_date: String) -> String {
    let day_hours = (NaiveDateTime::parse_from_str(&chosen_date, "%Y-%m-%d(%H:%M:%S)").unwrap() + Duration::days(1)).date().and_hms(0, 0, 0);
    let mut day = String::new();
    let mut looptime = NaiveDateTime::parse_from_str(&chosen_date, "%Y-%m-%d(%H:%M:%S)").unwrap().date().and_hms(0, 0, 0);

    let min_line_length = config_file.hours.min_line_length as usize;
    
    let hours_div = {
        if config_file.hours.hours > 24 {
            24
        } else {
           config_file.hours.hours 
        }
    };
    
    let text_format = &config_file.hours.text_format;

    let database_as_vec = files::read_from_database(config_file);

    while looptime < day_hours {
        day = day + &looptime.format(text_format).to_string();
        day.push_str(&config_file.hours.horizontal_divisor.repeat(min_line_length-text_format.len()));
        for i in database_as_vec.iter() {
            if (looptime <= NaiveDateTime::parse_from_str(&i.due, "%Y-%m-%d(%H:%M:%S)").unwrap()) &&
             (NaiveDateTime::parse_from_str(&i.due, "%Y-%m-%d(%H:%M:%S)").unwrap() <= looptime + Duration::hours(24)/(24/hours_div)) {
                day.push_str("\n");
                day.push_str(&config_file.hours.vertical_divisor);
                day.push_str(&i.task);
                day.push_str(&" ".repeat(min_line_length - 2 - i.task.len()));
                day.push_str(&config_file.hours.vertical_divisor);
            }
        }
        day.push_str("\n");

        looptime = looptime + Duration::hours(24)/(24/hours_div);
    }

    day.push_str(&"-".repeat(min_line_length));
    day
}

pub fn create_week(config_file: &Config, chosen_date: String) -> String {
    let mut weekdays = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let chosen_date_localdate = NaiveDateTime::parse_from_str(&chosen_date, "%Y-%m-%d(%H:%M:%S)").unwrap();
    let mut d_weekday = chosen_date_localdate.date().weekday().num_days_from_sunday();
    if config_file.week.starts_on_monday {
        d_weekday = chosen_date_localdate.date().weekday().num_days_from_monday();
        weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    }
    let mut week = String::new();
    let mut week_as_vec: Vec<String> = Vec::new();

    let mut max_lines: u32 = 0;

    for weekday in 0..7 {
        let day = create_hours(&config_file, (chosen_date_localdate + Duration::days(1)*((d_weekday + weekday - 6) as i32)).format("%Y-%m-%d(%H:%M:%S)").to_string());
        let day_as_vec = day.split("\n").collect::<Vec<&str>>();

        if day_as_vec.len() as u32 > max_lines {
            max_lines = day_as_vec.len() as u32;
        }
        week_as_vec.push(day.to_string());

        week.push_str(weekdays[weekday as usize]);
        week.push_str(&"-".repeat(&day_as_vec[0].len() - weekdays[weekday as usize].len()));
    }
    week.push_str("\n");

    for i in 0..max_lines {
        for day in week_as_vec.iter() {
            if day.split("\n").collect::<Vec<&str>>().len() > i as usize {
                week.push_str(
                    {
                        let x = day.split("\n").collect::<Vec<&str>>();
                        x[i as usize]
                    }
                );
            } else {
                week.push_str(&config_file.hours.vertical_divisor);
                week.push_str(&" ".repeat(config_file.hours.min_line_length as usize - 2));
                week.push_str(&config_file.hours.vertical_divisor);
            }
        }
        week.push_str("\n");
    }
    week
}