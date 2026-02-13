use std::path::PathBuf;
use directories::ProjectDirs;
use std::fs;


pub struct AppConfig {
    pub team: String,
}

// On recupere le chemin du dossier suivant l'OS
pub fn get_config_dir() -> Result<PathBuf, String> {
    let proj_dir = ProjectDirs::from("fr","aps","branchgen-cli")
    .ok_or("Unable to find config dir")?;

    let config_path = proj_dir.config_dir();
    println!("{:?}", config_path);
    fs::create_dir_all(config_path).map_err(|e| format!("Unable to create config dir: {}", e))?;
    Ok(config_path.to_path_buf())
}

pub fn get_config_file_path() -> Result<PathBuf, String> {
    let config_file = get_config_dir()?.join("config.json");
    Ok(config_file)
}

pub fn config_file_exists() -> Result<bool, String> {
    let config_file = get_config_file_path()?;
    config_file.try_exists().map_err(|e| format!("Check your configuration file: {}", e))
}