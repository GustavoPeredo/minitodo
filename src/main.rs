mod tables;
mod files;
mod config;

use chrono::prelude::Local;
use chrono::offset::TimeZone;
use chrono::{Duration, NaiveDate, NaiveDateTime};

use std::io::{self, BufRead};
use std::env;

use atty::Stream;

fn main() {
    let stdin = io::stdin();
    
    let mut config_file = files::get_config();

    let mut task = files::Task {
        task: String::from(""),
        due: Local::now().format("%Y-%m-%d(%H:%M:%S)").to_string(),
        //priority: String::from("A"),
    };

    let mut args: Vec<String> = env::args().collect();

    let mut input= String::from("");

    let mut pipe = String::from("");

    if !(atty::is(Stream::Stdin)) {
        for line in stdin.lock().lines() {
            let line = line.expect("Could not read line from standard in");
            pipe.push_str("\n");
            pipe = pipe + &line;
        }
        args.push(pipe);
    }

    for i in 1..(args.len()) {
        if &args[i].len() > &2 && &args[i][..2] == "--" {
            match &args[i][..] {
                "--hours" => {config_file.hours.hours = args[i+1].parse::<i32>().unwrap();},
                "--dateformat" => {config_file.hours.text_format = args[i+1].to_string().replace('\n', "");},
                "--weekformat" => {config_file.week.text_format = args[i+1].to_string().replace('\n', "");},
                "--minlinelength" => {config_file.hours.min_line_length = args[i+1].parse::<i32>().unwrap();},
                "--hd" => {config_file.hours.horizontal_divisor = args[i+1].as_bytes()[0].to_string();},
                "--week-vertical" => {config_file.week.show_vertically = true;},
                "--week-horizontal" => {config_file.week.show_vertically = false;},
                "--week-hd" => {config_file.week.horizontal_divisor = args[i+1].as_bytes()[0].to_string();},
                "--vd" => {config_file.hours.vertical_divisor = args[i+1].as_bytes()[0].to_string();},
                "--startsonsunday" => {config_file.week.starts_on_monday = false;},
                "--startsonmonday" => {config_file.week.starts_on_monday = true;},
                "--input" => {input = args[i+1].to_string();},
                //"--priority" => {task.priority = args[i+1].chars().nth(0).unwrap().to_string().to_uppercase();},
                "--task" => {task.task = args[i+1].to_string().replace('\n', "");},
                "--date" => {
                    if args[i+1].eq_ignore_ascii_case("Today") {
                        task.due = task.due;
                    } else if args[i+1].eq_ignore_ascii_case("Tommorow") {
                        task.due = (NaiveDateTime::parse_from_str(&task.due, "%Y-%m-%d(%H:%M:%S)").unwrap() + Duration::days(1)).format("%Y-%m-%d(%H:%M:%S)").to_string();
                    } else if args[i+1].eq_ignore_ascii_case("NextWeek") {
                        task.due = (NaiveDateTime::parse_from_str(&task.due, "%Y-%m-%d(%H:%M:%S)").unwrap() + Duration::days(7)).format("%Y-%m-%d(%H:%M:%S)").to_string();
                    } else {
                        match NaiveDate::parse_from_str(&args[i+1], "%Y-%m-%d") {
                            Ok(parsed) => {
                                task.due = TimeZone::from_local_datetime(&Local, &parsed.and_hms(0,0,0)).unwrap().format("%Y-%m-%d(%H:%M:%S)").to_string();
                            },
                            Err(_) => {
                                println!("Not properly formated, due should be formated as %Y-%m-%d")
                            }
                        }
                    }
                }
                other => {println!("{} is not a valid argumet", other);},
            }
        }
    }

    if args.iter().any(|i| i=="show") {
        if args.iter().any(|i| i=="week") {
            println!("{}", tables::create_week(&config_file, &task.due));
        } else if args.iter().any(|i| i=="day") {
            println!("{}", tables::create_hours(&config_file, &task.due));
        } else if args.iter().any(|i| i=="db") {
            let result = files::read_from_database(&config_file);
            for i in result.iter() {
                println!("{}", i.task);
            }
        } else {
            println!("Not a valid 'show' option");
        }
    } else if args.iter().any(|i| i=="update") {
        if args.iter().any(|i| i=="add") {
            if task.task != "" {
                files::write_to_database(
                    &config_file,
                    &task
                ).unwrap();
            } else {
                println!("Database not updated, no task was given")
            }
        } else if args.iter().any(|i| i=="remove") {
            files::remove_from_database(
                &config_file,
                &task
            ).unwrap();
            let result = files::read_from_database(&config_file);
            for i in result.iter() {
                println!("{}", i.task);
            }
        } else if args.iter().any(|i| i=="push") {
            files::update_database(&config_file, &tables::read_hours(&config_file, &task.due, &tables::create_hours(&config_file, &task.due)), &tables::read_hours(&config_file, &task.due, &input)).unwrap();
        }
    }
}
