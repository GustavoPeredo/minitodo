use crate::config;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::ErrorKind;
use serde_derive::Deserialize;
use toml;

use chrono::{DateTime, TimeZone};

#[derive(Deserialize)]
pub struct Config {
    pub hours: Hours,
    pub week: Week,
    pub database: Database,
}

#[derive(Deserialize)]
pub struct Hours {
    pub hours: i32,
    pub text_format: String,
    pub min_line_length: i32,
    pub horizontal_divisor: String,
    pub vertical_divisor: String,
}

#[derive(Deserialize)]
pub struct Week {
    pub starts_on_monday: bool,
}

#[derive(Deserialize)]
pub struct Database {
    pub path: String,
    pub filename: String,
}
 
pub struct Task<Tz: TimeZone> {
    pub task: String,
    pub due: DateTime<Tz>,
    pub priority: String, 
}

fn create_config_file() -> std::io::Result<()> {

    fs::create_dir_all(config::get_config_path()).unwrap();

    let mut file = File::create(
        Path::new(config::get_config_path()).
        join(config::get_config_name())
    ).unwrap();

    file.write_all(config::get_defaults().as_bytes()).unwrap();

    Ok(())
}

fn create_backup_file() -> std::io::Result<()> {
    fs::copy(
        Path::new(config::get_config_path()).
        join(config::get_config_name()),
        Path::new(config::get_config_path()).
        join(config::get_config_name().to_string() + ".bak")
    ).unwrap();

    create_config_file().unwrap();
    Ok(())
}

fn read_config_file() -> String {

    let file = File::open(
        Path::new(config::get_config_path()).
        join(config::get_config_name())
    );

    let mut buffer = String::new();

    match file {
        Ok(mut file) => {
            file.read_to_string(&mut buffer).unwrap();
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match create_config_file() {
                Ok(_) => {buffer = config::get_defaults().to_string();},
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    buffer
}

pub fn get_config() -> Config {
    let read_config = toml::from_str(&read_config_file());

    let config: Config = { match read_config {
            Ok(config) => config,
            Err(error) => {
                println!("Unexpected error happened while reading config file.");
                println!("{}", error);
                println!("\n Saving old configuration as backup and creating new default one...");
                create_backup_file().unwrap();
                toml::from_str(&read_config_file()).unwrap()
            }
        }
    };
    config
}

pub fn create_database_file(config_file: &Config) -> std::io::Result<()> {
    fs::create_dir_all(&config_file.database.path).unwrap();
    File::create(
        Path::new(&config_file.database.path).
        join(&config_file.database.filename)
    ).unwrap();

    Ok(())
}

pub fn write_to_database<Tz: TimeZone>(config_file: &Config, task: &Task<Tz>) -> std::io::Result<()> 
where Tz::Offset: std::fmt::Display,  {

    let file = File::open(
        Path::new(&config_file.database.path).
        join(&config_file.database.filename)
    );

    let mut buffer = String::new();

    match file {
        Ok(mut file) => {
            file.read_to_string(&mut buffer).unwrap();
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match create_database_file(config_file) {
                Ok(_) => {
                    buffer = "".to_string();
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    buffer.push_str("\n");
    buffer.push_str(&format!("({}) ", task.priority)[..]);
    buffer.push_str(&format!("{} ", task.task)[..]);
    buffer.push_str(&format!("due:{}", task.due.format("%Y-%m-%d(%H:%M:%S)").to_string())[..]);

    fs::write(
        Path::new(&config_file.database.path).
        join(&config_file.database.filename),
        buffer)
}
