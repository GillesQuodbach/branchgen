use crate::domain::types::{GeneratedOutput, HistoryFile, HistoryItem, WorkItemInput};

#[derive(Debug, Default)]
pub struct AppState {
    pub team_name: String,
    pub work_item_input: WorkItemInput,
    pub history: Vec<HistoryItem>,
    pub should_quit: bool,
    pub status: String,
}

impl AppState {
    pub fn new(team_name: String, history: Vec<HistoryItem>) -> Self {
        Self {
            team_name,
            work_item_input: WorkItemInput::default(),
            history,
            should_quit: false,
            status: "Ready".to_string(),
        }
    }
}