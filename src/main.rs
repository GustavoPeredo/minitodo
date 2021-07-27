mod tables;
mod files;
mod config;

use std::env;

fn main() {
    let config_file = files::get_config();

    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let command = &args[1];
        if command.eq_ignore_ascii_case("show") || command.eq_ignore_ascii_case("s") {
            if args.len() == 3 {
                let show = &args[2];
                if show.eq_ignore_ascii_case("week") || show.eq_ignore_ascii_case("w") {
                    println!("{}", tables::create_week(
                        &config_file
                    ));
                } else if show.eq_ignore_ascii_case("day") || show.eq_ignore_ascii_case("day") {
                    println!("{}", tables::create_hours(
                        &config_file.hours
                    ));
                }
            }
        }
    }
}
