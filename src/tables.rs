use crate::files::{self, Config, Task};
use chrono::{Duration, NaiveDateTime, NaiveTime, Datelike, Timelike};

pub fn create_hours(config_file: &Config, chosen_date: &String) -> String {

    let day_hours = (NaiveDateTime::parse_from_str(chosen_date, "%Y-%m-%d(%H:%M:%S)").unwrap() + Duration::days(1)).date().and_hms(0, 0, 0);
    let mut day = String::new();
    let mut looptime = NaiveDateTime::parse_from_str(chosen_date, "%Y-%m-%d(%H:%M:%S)").unwrap().date().and_hms(0, 0, 0);
    let min_line_length = config_file.hours.min_line_length as usize;
    
    let mut max_tasks_in_a_day = 0;
    let mut tasks_in_a_time: i32;

    let mut max_line_length: usize = 0;

    let mut day_dict_key: Vec<String> = Vec::new();
    let mut day_dict_value: Vec<String>;
    let mut day_dict_values: Vec<Vec<String>> = Vec::new();

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

        tasks_in_a_time = 0;

        day_dict_key.push(looptime.format(text_format).to_string());
        day_dict_value = Vec::new();

        for line in 0..database_as_vec.len() {
            if (looptime <= NaiveDateTime::parse_from_str(&database_as_vec[line].due, "%Y-%m-%d(%H:%M:%S)").unwrap()) &&
            (NaiveDateTime::parse_from_str(&database_as_vec[line].due, "%Y-%m-%d(%H:%M:%S)").unwrap() < looptime + Duration::hours(24)/(24/hours_div)) {

                tasks_in_a_time = tasks_in_a_time + 1;
                if database_as_vec[line].task.len() > max_line_length {
                    max_line_length = database_as_vec[line].task.len();
                }

                day_dict_value.push((&database_as_vec[line].task[..]).to_string());
            }
        }

        if tasks_in_a_time > max_tasks_in_a_day {
            max_tasks_in_a_day = tasks_in_a_time;
        }

        day_dict_values.push(day_dict_value);
        looptime = looptime + Duration::hours(24)/(24/hours_div);
    }

    if max_line_length < min_line_length {
        max_line_length = min_line_length as usize;
    }

    for i in 0..day_dict_key.len() {
        day.push_str(&day_dict_key[i]);
        day.push_str(&config_file.hours.horizontal_divisor.repeat(max_line_length - day_dict_key[i].len() + 2));
        for j in 0..max_tasks_in_a_day + 1 {
            day.push_str("\n");
            day.push_str(&config_file.hours.vertical_divisor);
            if j < day_dict_values[i].len() as i32 {
                day.push_str(&day_dict_values[i][j as usize]);
                if max_line_length - day_dict_values[i][j as usize].len() > 2 {
                    day.push_str(&" ".repeat(max_line_length - day_dict_values[i][j as usize].len()));
                }
            } else {
                day.push_str(&" ".repeat(max_line_length));
            }
            day.push_str(&config_file.hours.vertical_divisor);
        }
        day.push_str("\n");
    }
    day.push_str(&"-".repeat(max_line_length + 2));

    day
}

pub fn create_week(config_file: &Config, chosen_date: &String) -> String {
    let chosen_date_localdate = NaiveDateTime::parse_from_str(chosen_date, "%Y-%m-%d(%H:%M:%S)").unwrap();
    let mut d_weekday = chosen_date_localdate.date().weekday().num_days_from_sunday();
    if config_file.week.starts_on_monday {
        d_weekday = chosen_date_localdate.date().weekday().num_days_from_monday();
    }
    let mut week = String::new();
    let mut week_as_vec: Vec<String> = Vec::new();

    let mut max_lines: u32 = 0;

    for weekday in 0..7 {
        let day = create_hours(&config_file, &(chosen_date_localdate + Duration::days(1)*((weekday as i32) - (d_weekday as i32) - 1)).format("%Y-%m-%d(%H:%M:%S)").to_string());
        let day_as_vec = day.split("\n").collect::<Vec<&str>>();
        
        week.push_str(&(chosen_date_localdate + Duration::days(1)*((weekday as i32) - (d_weekday as i32) - 1)).format(&config_file.week.text_format).to_string());
        week.push_str(&config_file.week.horizontal_divisor.repeat(day_as_vec[0].len() - &(chosen_date_localdate + Duration::days(1)*((weekday as i32) - (d_weekday as i32) - 1)).format(&config_file.week.text_format).to_string().len()));
        if config_file.week.show_vertically {
            week.push_str("\n");
            week.push_str(&day.to_string());
            week.push_str("\n");
        } else {
            if day_as_vec.len() as u32 > max_lines {
                max_lines = day_as_vec.len() as u32;
            }
            week_as_vec.push(day.to_string());
        }
        
    }

    if !(config_file.week.show_vertically) {
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
                    week.push_str(&" ".repeat(config_file.hours.min_line_length as usize + 2));
                }
            }
            week.push_str("\n");
        }
    } 
    
    week
}

pub fn read_hours(config_file: &Config, chosen_date: &String, day_text: &String) -> Vec<Task> {
    let mut timestamps = NaiveDateTime::parse_from_str(chosen_date, "%Y-%m-%d(%H:%M:%S)").unwrap()
    .date().and_hms(0,0,0);

    let mut blank_chars: usize;

    let mut input_tasks: Vec<Task> = Vec::new();

    for mut line in day_text.lines() {
        blank_chars = 0;
        if line.len() > config_file.hours.text_format.len() {
            match NaiveTime::parse_from_str(&line[0..config_file.hours.text_format.len()], &config_file.hours.text_format) {
                Ok(time) => {
                    timestamps = NaiveDateTime::parse_from_str(chosen_date, "%Y-%m-%d(%H:%M:%S)").unwrap()
                    .date().and_hms(time.hour(), time.minute(), time.second());
                },
                Err(_) => {
                    if line[0..config_file.hours.text_format.len()] != config_file.hours.horizontal_divisor.repeat(config_file.hours.text_format.len()) {
                        if line.chars().nth(0) == config_file.hours.vertical_divisor.chars().nth(0) {
                            line = &line[1..];
                        }
                        let line = line.chars().rev().collect::<String>();
                        for ch in line.chars() {
                            if ch.to_string() == config_file.hours.vertical_divisor || ch.to_string() == " " {
                                blank_chars = blank_chars + 1;
                            } else {
                                break;
                            }
                        }
                        let line = &line[blank_chars..];
                        let line = line.chars().rev().collect::<String>();
                        if line.len() > 2 {
                            input_tasks.push(Task{
                                task: line,
                                due: timestamps.format("%Y-%m-%d(%H:%M:%S)").to_string(),
                            });
                        }
                    }
                },
            }
        } 
    }
    input_tasks
}

pub fn read_week(config_file: &Config, chosen_date: &String, day_text: &String) -> Vec<Task> {
    Vec::new()
}