use crate::app::input_mode::InputMode;
use crate::domain::types::{CommitType, HistoryItem, StoryType, WorkItemInput};
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
    pub error_message: Option<String>,
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
            error_message: None,
        }
    }

    pub fn insert_char_in_selected(&mut self, c: char) {
        match self.selected_field {
            Field::Pi => {
                if c.is_ascii_digit() {
                    let mut value = self
                        .work_item_input
                        .pi
                        .map(|v| v.to_string())
                        .unwrap_or_default();
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
                self.work_item_input.story_number.get_or_insert(String::new()).push(c);
            }
            Field::StoryTitle => {
                self.work_item_input.story_title.get_or_insert(String::new()).push(c);
            }
            Field::CommitMessage => {
                self.work_item_input.commit_message.get_or_insert(String::new()).push(c);
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
                if let Some(value) = self.work_item_input.story_number.as_mut() {
                    value.pop();
                        if value.is_empty() {
                            self.work_item_input.story_number = None;
                        }
                }
            }
            Field::StoryTitle => {
                if let Some(value) = self.work_item_input.story_title.as_mut() {
                    value.pop();
                    if value.is_empty() {
                        self.work_item_input.story_title = None;
                    }
                }
            }
            Field::CommitMessage => {
                if let Some(value) = self.work_item_input.commit_message.as_mut() {
                    value.pop();
                    if value.is_empty() {
                        self.work_item_input.commit_message = None;
                    }
                }
            }
            Field::StoryType | Field::CommitType | Field::Github | Field::History => {}
        }
    }

    pub fn validate_current_field(&self) -> Result<(), String> {
        match self.selected_field {
            Field::Pi => {
                if self.work_item_input.pi.is_none(){
                    Err("PI is missing".to_string())
                } else {
                    Ok(())
                }
            }
            Field::It => {
                if self.work_item_input.it.is_none(){
                    Err("IT is missing".to_string())
                } else {
                    Ok(())
                }
            }
            Field::StoryNumber => {
                let value = self.work_item_input.story_number.as_deref().ok_or("Story number is missing")?;
                WorkItemInput::validate_story_number(value)
            }
            Field::StoryTitle => {
                let value = self.work_item_input.story_title.as_deref().ok_or("Story title is missing")?;
                if value.trim().is_empty() {
                    Err("Story title is missing".to_string())
                } else {
                    Ok(())
                }
            }
            Field::CommitMessage => {
                let value = self.work_item_input.commit_message.as_deref().ok_or("Commit message is missing")?;
                if value.trim().is_empty() {
                    Err("Commit message is empty".to_string())
                } else {
                    Ok(())
                }
            }
            Field::StoryType => {
                if self.work_item_input.story_type.is_none(){
                    Err("Story type is missing".to_string())
                } else {
                    Ok(())
                }
            }
            Field::CommitType => {
                if self.work_item_input.commit_type.is_none(){
                    Err("Commit type is missing".to_string())
                } else {
                    Ok(())
                }
            }
            Field::Github | Field::History => Ok(())
        }
    }

    pub fn validate_all_fields(&self) -> Result<(), String> {
        self.work_item_input.validate()
    }

    pub fn select_next_in_selected(&mut self) {
        match self.selected_field {
            Field::It => {
                self.work_item_input.it = Some(match self.work_item_input.it{
                    None => 1,
                    Some(1) => 2,
                    Some(2) => 3,
                    Some(3) => 4,
                    Some(4) => 5,
                    Some(5) => 1,
                    Some(_) => 1,
                });
                self.error_message = None;
            }

        Field::StoryType => {
            self.work_item_input.story_type = Some(match self.work_item_input.story_type.as_ref() {
                None => StoryType::Feature,
                Some(value) => value.next(),
            });
            self.error_message = None;
        }
        Field::CommitType => {
            self.work_item_input.commit_type = Some(match self.work_item_input.commit_type.as_ref() {
                None => CommitType::Feat,
                Some(value) => value.next(),
            });
            self.error_message = None;
        }
        _ => {}
        }
    }

    pub fn select_prev_in_selected(&mut self) {
        match self.selected_field {
            Field::It => {
                self.work_item_input.it = Some(match self.work_item_input.it{
                    None => 5,
                    Some(5) => 4,
                    Some(4) => 3,
                    Some(3) => 2,
                    Some(2) => 1,
                    Some(1) => 5,
                    Some(_) => 5,
                });
                self.error_message = None;
            }

            Field::StoryType => {
                self.work_item_input.story_type = Some(match self.work_item_input.story_type.as_ref() {
                    None => StoryType::Test,
                    Some(value) => value.prev(),
                });
                self.error_message = None;
            }
            Field::CommitType => {
                self.work_item_input.commit_type = Some(match self.work_item_input.commit_type.as_ref() {
                    None => CommitType::Ops,
                    Some(value) => value.prev(),
                });
                self.error_message = None;
            }
            _ => {}
        }
    }
}