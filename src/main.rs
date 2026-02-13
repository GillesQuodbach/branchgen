mod app;
mod domain;
mod ui;
mod storage;
mod error;
mod config;

use crate::config::app_config::{get_config_dir, AppConfig};


fn main() {
    println!("*********************");
    println!("Welcome to BranchGen");
    println!("*********************");

    let config_dir = get_config_dir().expect("Unable to find config dir");
    
}
