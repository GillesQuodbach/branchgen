use crate::domain::types::{GeneratedOutput, HistoryFile, HistoryItem, WorkItemInput};

#[derive(Debug, Default)]
pub struct AppState {
    pub work_item_input: WorkItemInput,
    pub history: Vec<HistoryItem>,
    pub should_quit: bool,
    pub status: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            work_item_input: WorkItemInput::default(),
            history: Vec::new(),
            should_quit: false,
            status: "Ready".to_string(),
        }
    }
}