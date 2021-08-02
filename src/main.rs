mod tables;
mod files;
mod config;

use chrono::prelude::Local;
use chrono::Duration;
use chrono::NaiveDate;
use chrono::offset::TimeZone;

use std::io;
use std::io::BufRead;
use std::env;

use atty::Stream;

fn main() {
    let stdin = io::stdin();
    
    let mut config_file = files::get_config();

    let mut task = files::Task {
        task: String::from(""),
        due: Local::now(),
        priority: String::from("A"),
    };

    let mut args: Vec<String> = env::args().collect();

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
                "--minlinelength" => {config_file.hours.min_line_length = args[i+1].parse::<i32>().unwrap();},
                "--hd" => {config_file.hours.horizontal_divisor = args[i+1].to_string().replace('\n', "");},
                "--vd" => {config_file.hours.vertical_divisor = args[i+1].to_string().replace('\n', "");},
                "--priority" => {task.priority = args[i+1].chars().nth(0).unwrap().to_string().to_uppercase();},
                "--task" => {task.task = args[i+1].to_string().replace('\n', "");},
                "--due" => {
                    if args[i+1].eq_ignore_ascii_case("Today") {
                        task.due = task.due;
                    } else if args[i+1].eq_ignore_ascii_case("Tommorow") {
                        task.due = task.due + Duration::hours(24);
                    } else if args[i+1].eq_ignore_ascii_case("NextWeek") {
                        task.due = task.due + Duration::days(7);
                    } else {
                        match NaiveDate::parse_from_str(&args[i+1], "%Y-%m-%d") {
                            Ok(parsed) => {
                                task.due = TimeZone::from_local_datetime(&Local, &parsed.and_hms(0,0,0)).unwrap();
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
            println!("{}", tables::create_week(&config_file, Local::now().format("%Y-%m-%d(%H:%M:%S)").to_string()));
        } else if args.iter().any(|i| i=="day") {
            println!("{}", tables::create_hours(&config_file, Local::now().format("%Y-%m-%d(%H:%M:%S)").to_string()));
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
                task.task
            ).unwrap();
            let result = files::read_from_database(&config_file);
            for i in result.iter() {
                println!("{}", i.task);
            }
        }
    }
}
