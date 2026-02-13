mod app;
mod domain;
mod ui;
mod storage;
mod error;
mod config;

use crate::config::app_config::{config_file_exists, create_config_file, get_config_dir, AppConfig};
use std::io::{ stdin, stdout, Write};

fn main() {
    println!("*********************");
    println!("Welcome to BranchGen");
    println!("*********************");

    let config_file_exists = match config_file_exists() {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error while checking config file: {e}");
            return;
        }
    };

    if config_file_exists {
        println!("Config file already exists");
        return;
    }
        let mut input_team = String::new();
        println!("It's your first launch");
        println!("Please enter your team name");
        stdin().read_line(&mut input_team).unwrap();
        let team_name = input_team.trim().to_uppercase();

        if let Err(e) = create_config_file(team_name) {
            eprintln!("Error while creating config file: {e}");
        }


}
