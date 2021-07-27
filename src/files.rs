use crate::config;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use std::io::ErrorKind;
use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub hours: Hours,
    pub week: Week,
}

#[derive(Deserialize)]
pub struct Hours {
    pub hours: i32,
    pub text_format: String,
    pub line_length: i32,
}

#[derive(Deserialize)]
pub struct Week {
    pub starts_on_monday: bool,
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
    //            create_backup_file().unwrap();
    //buffer = config::get_defaults().to_string();
    let config: Config = toml::from_str(&read_config_file()).unwrap();
    config
}

