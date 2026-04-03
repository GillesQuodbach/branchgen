use crate::app::AppState;
use crate::domain::field::Field;
use crate::domain::types::{CommitType, StoryType};

pub fn insert_char_in_selected(state: &mut AppState, c: char) {
    match state.selected_field {
        Field::Pi => {
            if c.is_ascii_digit() {
                let mut value = state
                    .work_item_input
                    .pi
                    .map(|v| v.to_string())
                    .unwrap_or_default();
                value.push(c);

                if let Ok(parsed) = value.parse::<u32>() {
                    state.work_item_input.pi = Some(parsed);
                }
            }
        }
        Field::StoryNumber => {
            state.work_item_input.story_number.get_or_insert(String::new()).push(c);
        }
        Field::StoryTitle => {
            state.work_item_input.story_title.get_or_insert(String::new()).push(c);
        }
        Field::CommitMessage => {
            state.work_item_input.commit_message.get_or_insert(String::new()).push(c);
        }
        Field::It | Field::StoryType | Field::CommitType | Field::Validate | Field::Github | Field::History => {}
    }
}

pub fn select_prev_in_selected(state: &mut AppState) {
    match state.selected_field {
        Field::It => {
            state.work_item_input.it = Some(match state.work_item_input.it{
                None => 5,
                Some(5) => 4,
                Some(4) => 3,
                Some(3) => 2,
                Some(2) => 1,
                Some(1) => 5,
                Some(_) => 5,
            });
            state.error_message = None;
        }

        Field::StoryType => {
            state.work_item_input.story_type = Some(match state.work_item_input.story_type.as_ref() {
                None => StoryType::Test,
                Some(value) => value.prev(),
            });
            state.error_message = None;
        }
        Field::CommitType => {
            state.work_item_input.commit_type = Some(match state.work_item_input.commit_type.as_ref() {
                None => CommitType::Ops,
                Some(value) => value.prev(),
            });
            state.error_message = None;
        }
        _ => {}
    }
}

pub fn select_next_in_selected(state: &mut AppState) {
    match state.selected_field {
        Field::It => {
            state.work_item_input.it = Some(match state.work_item_input.it{
                None => 1,
                Some(1) => 2,
                Some(2) => 3,
                Some(3) => 4,
                Some(4) => 5,
                Some(5) => 1,
                Some(_) => 1,
            });
            state.error_message = None;
        }

        Field::StoryType => {
            state.work_item_input.story_type = Some(match state.work_item_input.story_type.as_ref() {
                None => StoryType::Feature,
                Some(value) => value.next(),
            });
            state.error_message = None;
        }
        Field::CommitType => {
            state.work_item_input.commit_type = Some(match state.work_item_input.commit_type.as_ref() {
                None => CommitType::Feat,
                Some(value) => value.next(),
            });
            state.error_message = None;
        }
        _ => {}
    }
}

pub fn backspace_in_selected(state: &mut AppState) {
    match state.selected_field {
        Field::Pi => {
            let mut value = state.work_item_input.pi.map(|v| v.to_string()).unwrap_or_default();
            value.pop();
            if value.is_empty(){
                state.work_item_input.pi = None;
            } else if let Ok(parsed) = value.parse::<u32>(){
                state.work_item_input.pi = Some(parsed);
            }
        }
        Field::StoryNumber => {
            if let Some(value) = state.work_item_input.story_number.as_mut() {
                value.pop();
                if value.is_empty() {
                    state.work_item_input.story_number = None;
                }
            }
        }
        Field::StoryTitle => {
            if let Some(value) = state.work_item_input.story_title.as_mut() {
                value.pop();
                if value.is_empty() {
                    state.work_item_input.story_title = None;
                }
            }
        }
        Field::CommitMessage => {
            if let Some(value) = state.work_item_input.commit_message.as_mut() {
                value.pop();
                if value.is_empty() {
                    state.work_item_input.commit_message = None;
                }
            }
        }
        Field::It | Field::StoryType | Field::CommitType | Field::Validate | Field::Github | Field::History => {}
    }
}