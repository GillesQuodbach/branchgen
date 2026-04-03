use std::path::PathBuf;
use crate::app::input_mode::InputMode;
use crate::domain::types::{GeneratedOutput, HistoryFile,WorkItemInput};
use crate::domain::field::Field;

#[derive(Debug, Default)]
pub struct AppState {
    pub selected_field: Field,
    pub input_mode: InputMode,
    pub team_name: String,
    pub work_item_input: WorkItemInput,
    pub generated_output: Option<GeneratedOutput>,
    pub history: HistoryFile,
    pub history_file_path: PathBuf,
    pub should_quit: bool,
    pub status: String,
    pub error_message: Option<String>,
}

impl AppState {
    // coeur de l'application
    // contient tout ce que l'ecran doit afficher et tout ce que l'utilisateur est
    // entrain de faire
    pub fn new(team_name: String, history: HistoryFile, history_file_path: PathBuf) -> Self {
        Self {
            selected_field: Field::StoryNumber,
            input_mode: InputMode::default(),
            team_name,
            work_item_input: WorkItemInput::default(),
            generated_output: None,
            history,
            history_file_path,
            should_quit: false,
            status: "Ready".to_string(),
            error_message: None,
        }
    }
}