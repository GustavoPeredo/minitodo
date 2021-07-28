mod tables;
mod files;
mod config;

use std::env;

fn main() {
    let mut config_file = files::get_config();

    let args: Vec<String> = env::args().collect();

    for i in 1..((args.len()+1)/2) {
        match &args[i*2 - 1][..] {
            "--hours" => {config_file.hours.hours = args[i*2].parse::<i32>().unwrap()},
            "--dateformat" => {config_file.hours.text_format = args[i*2].to_string()},
            "--linelength" => {config_file.hours.line_length = args[i*2].parse::<i32>().unwrap()},
            "show" => {
                if &args[i*2] == "week" {
                    println!("{}", tables::create_week(&config_file));
                } else if &args[i*2] == "day" {
                    println!("{}", tables::create_hours(&config_file));
                }
            }
            _ => {println!("Non recognized format")},
        }
    }
}
