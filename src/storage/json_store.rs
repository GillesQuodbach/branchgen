use std::fs;
use std::path::PathBuf;
use crate::config::app_config::{get_config_dir};
use crate::domain::types::{HistoryFile, HistoryItem};

// recuperation du chemin du fichier
pub fn get_history_file_path() -> Result<PathBuf,String> {
    let history_file = get_config_dir()?.join("history.json");
    Ok(history_file)
}

pub fn append_history_item(history_item: HistoryItem) -> Result<(), String> {
    let path = get_history_file_path()?;

    // on charge l'historique existant (ou créer si vide)
    let mut history: HistoryFile = if path.try_exists().map_err(|e| format!("Unable to check your history file: {}", e))?
    {
        let json = fs::read_to_string(&path).map_err(|e| format!("Unable to read history file: {}", e))?;

        if json.trim().is_empty() {
            HistoryFile::new(1, Vec::new())
        } else {
            serde_json::from_str(&json).map_err(|e| format!("Unable to deserialize history file: {}", e))?
        }

    } else {
        HistoryFile::new(1, Vec::new())
    };

    // push de l'history item
    history.push_item(history_item);

    // on reecrit le fichier complet
    let json = serde_json::to_string_pretty(&history).map_err(|e| format!("Unable to serialize history: {}", e))?;

    fs::write(path, json).map_err(|e| format!("Unable to write history file: {}", e))?;

    Ok(())

}