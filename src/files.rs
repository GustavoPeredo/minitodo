use crate::config;

use std::fs::{self, File};
use std::path::Path;
use std::io::{Write, Read, ErrorKind};

use serde_derive::Deserialize;
use toml;

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
    pub num_lines: i32,
    pub horizontal_divisor: String,
    pub vertical_divisor: String,
}

#[derive(Deserialize)]
pub struct Week {
    pub starts_on_monday: bool,
    pub horizontal_divisor: String,
    pub today_char: String,
    pub text_format: String,
}

#[derive(Deserialize)]
pub struct Database {
    pub path: String,
    pub filename: String,
}

pub struct Task{
    pub task: String,
    pub due: String,
    // pub priority: String, 
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

pub fn write_to_database(config_file: &Config, task: &Task) -> std::io::Result<()> {

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
    //buffer.push_str(&format!("({}) ", task.priority)[..]);
    buffer.push_str(&format!("{} ", task.task)[..]);
    buffer.push_str(&format!("due:{}", task.due)[..]);

    fs::write(
        Path::new(&config_file.database.path).
        join(&config_file.database.filename),
        buffer
    )
}

pub fn remove_from_database(config_file: &Config, task_name: String) -> std::io::Result<()> {

    let database_as_vec: Vec<Task> = read_from_database(config_file);

    let file = File::open(
        Path::new(&config_file.database.path).
        join(&config_file.database.filename)
    );

    let mut buffer = String::new();

    match file {
        Ok(mut file) => {
            file.read_to_string(&mut buffer).unwrap();
        },
        Err(error) => {
            panic!("Error: {}", error);
        }
    }
    
    let mut buffer_vector: Vec<&str> = buffer.split("\n").collect();

    for task in 0..database_as_vec.len() {
        if database_as_vec[task].task.trim().to_string() == task_name.trim().to_string() {
            buffer_vector.remove(task);
        }
    }

    buffer = buffer_vector.join("\n");

    fs::write(
        Path::new(&config_file.database.path).
        join(&config_file.database.filename),
        buffer
    )

}

pub fn read_from_database(config_file: &Config) -> Vec<Task> {
    let mut line_split: Vec<&str>;

    let file = fs::read_to_string(
        Path::new(&config_file.database.path).
        join(&config_file.database.filename)).unwrap();

    let database: Vec<&str> = file.split("\n").collect();

    let mut database_as_vec: Vec<Task> = Vec::new();

    for i in 0..(database.len()) {
        line_split = database[i].split(" due:").collect();
        if line_split.len() > 1 {
            database_as_vec.push(
                Task {
                    task: line_split[0].to_string(),
                    due: line_split[1].to_string(),
                }
            );
        }
    }
    database_as_vec
}

pub fn update_database(config_file: &Config, remove_tasks: &Vec<Task>, new_tasks: &Vec<Task>) -> std::io::Result<()> {
    for i in remove_tasks.iter() {
        match remove_from_database(config_file, i.task.to_string()) {
            Ok(()) => {},
            Err(_) => {println!("Unable to remove {} fromd database", i.task);},
        }
    }
    for i in new_tasks.iter() {
        match write_to_database(config_file, i) {
            Ok(()) => {},
            Err(_) => {println!("Unable to remove {} fromd database", i.task);},
        }
    }
    Ok(())
}