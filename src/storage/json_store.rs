use std::fs;
use std::path::{Path, PathBuf};
use crate::config::app_config::get_config_dir;
use crate::domain::types::{HistoryFile};

// recuperation du chemin du fichier
pub fn get_history_file_path() -> Result<PathBuf,String> {
    let history_file = get_config_dir()?.join("history.json");
    Ok(history_file)
}

// charge le fichier d'historique
pub fn load_history_file_from_path(path: &Path) -> Result<HistoryFile,String> {

    let json_exists: bool = path.try_exists().map_err(|e| format!("Unable to check your history file: {}", e))?;

    if !json_exists {
        return Ok(empty_history());
    }

    let content = fs::read_to_string(path).map_err(|e| format!("Unable to read history file: {}", e))?;

    if content.trim().is_empty() {
        return Ok(empty_history());
    }
    let history: HistoryFile =serde_json::from_str(&content).map_err(|e| format!("Unable to deserialize history file: {}", e))?;

    Ok(history)

}

// creation d' un nouveau fichier
fn empty_history() -> HistoryFile {
    HistoryFile::new(Vec::new())
}

// sauvegarde de l' historique
pub fn save_history_file(path: &Path, history: &HistoryFile) -> Result<(), String> {
    let json = serde_json::to_string_pretty(history).map_err(|e| format!("Failed to serialize history: {}", e))?;

    fs::write(path, json).map_err(|e| format!("Failed to save history: {}", e))?;

    Ok(())
}

