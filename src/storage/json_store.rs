use std::fs;
use std::path::PathBuf;
use crate::config::app_config::{get_config_dir};
use crate::domain::types::{HistoryFile, HistoryItem};

// recuperation du chemin du fichier
pub fn get_history_file_path() -> Result<PathBuf,String> {
    let history_file = get_config_dir()?.join("history.json");
    println!("History file path: {}", history_file.display());
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
    pub fn load_history_file() -> Result<HistoryFile,String> {
        let path = get_history_file_path()?;
        let json_exists: bool = path.try_exists().map_err(|e| format!("Unable to check your history file: {}", e))?;

        if !json_exists {
            return Ok(HistoryFile::new(1, Vec::new()));
        }

        let content = fs::read_to_string(&path).map_err(|e| format!("Unable to read history file: {}", e))?;

        if content.trim().is_empty() {
            return Ok(HistoryFile::new(1, Vec::new()));
        }
        serde_json::from_str(&content).map_err(|e| format!("Unable to deserialize history file: {}", e))?

    }

pub fn print_history(limit: Option<usize>) -> Result<(), String> {
    let history = load_history_file()?;
    let mut items: Vec<&HistoryItem> = history.items().iter().collect();

    if items.is_empty() {
        println!("No history items found.");
        return Ok(());
    }
    items.reverse();

    if let Some(limit) = limit {
        items.truncate(limit);
    }

    println!("History (version {}, {} entries)", history.version(), items.len());
    println!("===============================================================");

    for (index, item) in items.iter().enumerate() {
        println!("{}. {}", index + 1, item);
        println!("===============================================================");
    }
    Ok(())
}
