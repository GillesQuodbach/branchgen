
use crate::domain::types::{GeneratedOutput, HistoryFile, HistoryItem, WorkItemInput};
use crate::domain::field::Field;

#[derive(Debug, Default)]
pub struct AppState {
    pub selected_field: Field,
    pub team_name: String,
    pub work_item_input: WorkItemInput,
    pub history: Vec<HistoryItem>,
    pub should_quit: bool,
    pub status: String,
}

impl AppState {
    pub fn new(team_name: String, history: Vec<HistoryItem>) -> Self {
        Self {
            selected_field: Field::StoryNumber,
            team_name,
            work_item_input: WorkItemInput::default(),
            history,
            should_quit: false,
            status: "Ready".to_string(),
        }
    }
}