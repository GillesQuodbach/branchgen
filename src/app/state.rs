use std::path::PathBuf;
use crate::domain::types::{GeneratedOutput, HistoryFile,WorkItemInput};
use crate::domain::field::Field;

#[derive(Debug, Default)]
pub struct AppState {
    pub selected_field: Field,
    pub team_name: String,
    pub work_item_input: WorkItemInput,
    pub text_editors: TextEditors,
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
            team_name,
            work_item_input: WorkItemInput::default(),
            text_editors: Default::default(),
            generated_output: None,
            history,
            history_file_path,
            should_quit: false,
            status: "Ready".to_string(),
            error_message: None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct TextEditor {
    pub cursor: usize,
}
#[derive(Debug, Default, Clone)]
pub struct TextEditors {
    pub pi: TextEditor,
    pub story_number: TextEditor,
    pub story_title: TextEditor,
    pub commit_message: TextEditor,
}