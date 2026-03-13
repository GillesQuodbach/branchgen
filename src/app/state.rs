use crate::app::input_mode::InputMode;
use crate::domain::types::{HistoryItem, WorkItemInput};
use crate::domain::field::Field;

#[derive(Debug, Default)]
pub struct AppState {
    pub selected_field: Field,
    pub input_mode: InputMode,
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
            input_mode: InputMode::default(),
            team_name,
            work_item_input: WorkItemInput::default(),
            history,
            should_quit: false,
            status: "Ready".to_string(),
        }
    }

    pub fn insert_char_in_selected(&mut self, c: char) {
        match self.selected_field {
            Field::Pi => {
                if c.is_ascii_digit() {
                    let mut value = self.work_item_input.pi.map(|v| v.to_string()).unwrap_or_default();
                    value.push(c);

                    if let Ok(parsed) = value.parse::<u32>() {
                        self.work_item_input.pi = Some(parsed);
                    }
                }
            }
            Field::It => {
                if c.is_ascii_digit() {
                    let mut value = self.work_item_input.it.map(|v| v.to_string()).unwrap_or_default();
                    value.push(c);

                    if let Ok(parsed) = value.parse::<u32>() {
                        self.work_item_input.it = Some(parsed);
                    }
                }
            }
            Field::StoryNumber => {
                self.work_item_input.story_number.push(c);
            }
            Field::StoryTitle => {
                self.work_item_input.story_title.push(c);
            }
            Field::CommitMessage => {
                self.work_item_input.commit_message.push(c);
            }
            Field::StoryType | Field::CommitType | Field::Github | Field::History => {}
        }
    }

    pub fn backspace_in_selected(&mut self) {
        match self.selected_field {
            Field::Pi => {
                let mut value = self.work_item_input.pi.map(|v| v.to_string()).unwrap_or_default();
                value.pop();
                if value.is_empty(){
                    self.work_item_input.pi = None;
                } else if let Ok(parsed) = value.parse::<u32>(){
                    self.work_item_input.pi = Some(parsed);
                }
            }
            Field::It => {
                let mut value = self.work_item_input.it.map(|v| v.to_string()).unwrap_or_default();
                value.pop();
                if value.is_empty(){
                    self.work_item_input.it = None;
                } else if let Ok(parsed) = value.parse::<u32>(){
                    self.work_item_input.it = Some(parsed);
                }
            }
            Field::StoryNumber => {
                self.work_item_input.story_number.pop();
            }
            Field::StoryTitle => {
                self.work_item_input.story_title.pop();
            }
            Field::CommitMessage => {
                self.work_item_input.commit_message.pop();
            }
            Field::StoryType | Field::CommitType | Field::Github | Field::History => {}
        }
    }
}