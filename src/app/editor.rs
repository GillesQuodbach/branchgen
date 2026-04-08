use crate::app::AppState;
use crate::domain::field::Field;
use crate::domain::types::{CommitType, StoryType};

fn char_count(s: &str) -> usize {
    s.chars().count()
}

fn char_to_byte_index(s: &str, char_index: usize) -> usize {
    s.char_indices()
        .nth(char_index)
        .map(|(idx, _)| idx)
        .unwrap_or(s.len())
}

fn clamp_cursor(cursor: &mut usize, s: &str) {
    let len = char_count(s);
    if *cursor > len {
        *cursor = len;
    }
}

fn insert_at(s: &mut String, cursor: &mut usize, c: char) {
    clamp_cursor(cursor, s);
    let byte_index = char_to_byte_index(s, *cursor);
    s.insert(byte_index, c);
    *cursor += 1;
}

fn backspace_at(s: &mut String, cursor: &mut usize) {
    clamp_cursor(cursor, s);

    if *cursor == 0 {
        return;
    }

    let end = char_to_byte_index(s, *cursor);
    let start = char_to_byte_index(s, *cursor - 1);
    s.replace_range(start..end, "");
    *cursor -= 1;
}

fn delete_at(s: &mut String, cursor: &mut usize) {
    clamp_cursor(cursor, s);

    let len = char_count(s);
    if *cursor >= len {
        return;
    }

    let start = char_to_byte_index(s, *cursor);
    let end = char_to_byte_index(s, *cursor + 1);
    s.replace_range(start..end, "");
}

fn move_cursor_left(cursor: &mut usize) {
    if *cursor > 0 {
        *cursor -= 1;
    }
}

fn move_cursor_right(cursor: &mut usize, s: &str) {
    let len = char_count(s);
    if *cursor < len {
        *cursor += 1;
    }
}

fn move_cursor_home(cursor: &mut usize) {
    *cursor = 0;
}

fn move_cursor_end(cursor: &mut usize, s: &str) {
    *cursor = char_count(s);
}

fn pi_as_string(state: &AppState) -> String {
    state
        .work_item_input
        .pi
        .map(|v| v.to_string())
        .unwrap_or_default()
}

fn write_pi_from_string(state: &mut AppState, value: String) {
    if value.is_empty() {
        state.work_item_input.pi = None;
        return;
    }

    if let Ok(parsed) = value.parse::<u32>() {
        state.work_item_input.pi = Some(parsed);
    }
}

pub fn insert_char_in_selected(state: &mut AppState, c: char) {
    match state.selected_field {
        Field::Pi => {
            if c.is_ascii_digit() {
                let mut value = pi_as_string(state);
                insert_at(&mut value, &mut state.text_editors.pi.cursor, c);
                write_pi_from_string(state, value);
                state.error_message = None;
            }
        }
        Field::StoryNumber => {
            if c.is_ascii_alphanumeric() || c == '-' {
                let mut value = state.work_item_input.story_number.take().unwrap_or_default();
                insert_at(
                    &mut value,
                    &mut state.text_editors.story_number.cursor,
                    c,
                );
                state.work_item_input.story_number = if value.is_empty() {
                    None
                } else {
                    Some(value)
                };
                state.error_message = None;
            }
        }
        Field::StoryTitle => {
            let mut value = state.work_item_input.story_title.take().unwrap_or_default();
            insert_at(&mut value, &mut state.text_editors.story_title.cursor, c);
            state.work_item_input.story_title = if value.is_empty() {
                None
            } else {
                Some(value)
            };
            state.error_message = None;
        }
        Field::CommitMessage => {
            let mut value = state.work_item_input.commit_message.take().unwrap_or_default();
            insert_at(
                &mut value,
                &mut state.text_editors.commit_message.cursor,
                c,
            );
            state.work_item_input.commit_message = if value.is_empty() {
                None
            } else {
                Some(value)
            };
            state.error_message = None;
        }
        Field::It
        | Field::StoryType
        | Field::CommitType
        | Field::Validate
        | Field::Github
        | Field::History => {}
    }
}

pub fn backspace_in_selected(state: &mut AppState) {
    match state.selected_field {
        Field::Pi => {
            let mut value = pi_as_string(state);
            backspace_at(&mut value, &mut state.text_editors.pi.cursor);
            write_pi_from_string(state, value);
            state.error_message = None;
        }
        Field::StoryNumber => {
            let mut value = state.work_item_input.story_number.take().unwrap_or_default();
            backspace_at(
                &mut value,
                &mut state.text_editors.story_number.cursor,
            );
            state.work_item_input.story_number = if value.is_empty() {
                None
            } else {
                Some(value)
            };
            state.error_message = None;
        }
        Field::StoryTitle => {
            let mut value = state.work_item_input.story_title.take().unwrap_or_default();
            backspace_at(&mut value, &mut state.text_editors.story_title.cursor);
            state.work_item_input.story_title = if value.is_empty() {
                None
            } else {
                Some(value)
            };
            state.error_message = None;
        }
        Field::CommitMessage => {
            let mut value = state.work_item_input.commit_message.take().unwrap_or_default();
            backspace_at(
                &mut value,
                &mut state.text_editors.commit_message.cursor,
            );
            state.work_item_input.commit_message = if value.is_empty() {
                None
            } else {
                Some(value)
            };
            state.error_message = None;
        }
        Field::It
        | Field::StoryType
        | Field::CommitType
        | Field::Validate
        | Field::Github
        | Field::History => {}
    }
}

pub fn delete_in_selected(state: &mut AppState) {
    match state.selected_field {
        Field::Pi => {
            let mut value = pi_as_string(state);
            delete_at(&mut value, &mut state.text_editors.pi.cursor);
            write_pi_from_string(state, value);
            state.error_message = None;
        }
        Field::StoryNumber => {
            let mut value = state.work_item_input.story_number.take().unwrap_or_default();
            delete_at(
                &mut value,
                &mut state.text_editors.story_number.cursor,
            );
            state.work_item_input.story_number = if value.is_empty() {
                None
            } else {
                Some(value)
            };
            state.error_message = None;
        }
        Field::StoryTitle => {
            let mut value = state.work_item_input.story_title.take().unwrap_or_default();
            delete_at(&mut value, &mut state.text_editors.story_title.cursor);
            state.work_item_input.story_title = if value.is_empty() {
                None
            } else {
                Some(value)
            };
            state.error_message = None;
        }
        Field::CommitMessage => {
            let mut value = state.work_item_input.commit_message.take().unwrap_or_default();
            delete_at(
                &mut value,
                &mut state.text_editors.commit_message.cursor,
            );
            state.work_item_input.commit_message = if value.is_empty() {
                None
            } else {
                Some(value)
            };
            state.error_message = None;
        }
        Field::It
        | Field::StoryType
        | Field::CommitType
        | Field::Validate
        | Field::Github
        | Field::History => {}
    }
}

pub fn move_cursor_left_in_selected(state: &mut AppState) {
    match state.selected_field {
        Field::Pi => move_cursor_left(&mut state.text_editors.pi.cursor),
        Field::StoryNumber => move_cursor_left(&mut state.text_editors.story_number.cursor),
        Field::StoryTitle => move_cursor_left(&mut state.text_editors.story_title.cursor),
        Field::CommitMessage => move_cursor_left(&mut state.text_editors.commit_message.cursor),
        Field::It
        | Field::StoryType
        | Field::CommitType
        | Field::Validate
        | Field::Github
        | Field::History => {}
    }
}

pub fn move_cursor_right_in_selected(state: &mut AppState) {
    match state.selected_field {
        Field::Pi => {
            let value = pi_as_string(state);
            move_cursor_right(&mut state.text_editors.pi.cursor, &value);
        }
        Field::StoryNumber => {
            let value = state.work_item_input.story_number.as_deref().unwrap_or("");
            move_cursor_right(&mut state.text_editors.story_number.cursor, value);
        }
        Field::StoryTitle => {
            let value = state.work_item_input.story_title.as_deref().unwrap_or("");
            move_cursor_right(&mut state.text_editors.story_title.cursor, value);
        }
        Field::CommitMessage => {
            let value = state.work_item_input.commit_message.as_deref().unwrap_or("");
            move_cursor_right(&mut state.text_editors.commit_message.cursor, value);
        }
        Field::It
        | Field::StoryType
        | Field::CommitType
        | Field::Validate
        | Field::Github
        | Field::History => {}
    }
}

pub fn move_cursor_home_in_selected(state: &mut AppState) {
    match state.selected_field {
        Field::Pi => move_cursor_home(&mut state.text_editors.pi.cursor),
        Field::StoryNumber => move_cursor_home(&mut state.text_editors.story_number.cursor),
        Field::StoryTitle => move_cursor_home(&mut state.text_editors.story_title.cursor),
        Field::CommitMessage => move_cursor_home(&mut state.text_editors.commit_message.cursor),
        Field::It
        | Field::StoryType
        | Field::CommitType
        | Field::Validate
        | Field::Github
        | Field::History => {}
    }
}

pub fn move_cursor_end_in_selected(state: &mut AppState) {
    match state.selected_field {
        Field::Pi => {
            let value = pi_as_string(state);
            move_cursor_end(&mut state.text_editors.pi.cursor, &value);
        }
        Field::StoryNumber => {
            let value = state.work_item_input.story_number.as_deref().unwrap_or("");
            move_cursor_end(&mut state.text_editors.story_number.cursor, value);
        }
        Field::StoryTitle => {
            let value = state.work_item_input.story_title.as_deref().unwrap_or("");
            move_cursor_end(&mut state.text_editors.story_title.cursor, value);
        }
        Field::CommitMessage => {
            let value = state.work_item_input.commit_message.as_deref().unwrap_or("");
            move_cursor_end(&mut state.text_editors.commit_message.cursor, value);
        }
        Field::It
        | Field::StoryType
        | Field::CommitType
        | Field::Validate
        | Field::Github
        | Field::History => {}
    }
}

pub fn select_prev_in_selected(state: &mut AppState) {
    match state.selected_field {
        Field::It => {
            state.work_item_input.it = Some(match state.work_item_input.it {
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
            state.work_item_input.commit_type =
                Some(match state.work_item_input.commit_type.as_ref() {
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
            state.work_item_input.it = Some(match state.work_item_input.it {
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
            state.work_item_input.story_type =
                Some(match state.work_item_input.story_type.as_ref() {
                    None => StoryType::Feature,
                    Some(value) => value.next(),
                });
            state.error_message = None;
        }
        Field::CommitType => {
            state.work_item_input.commit_type =
                Some(match state.work_item_input.commit_type.as_ref() {
                    None => CommitType::Feat,
                    Some(value) => value.next(),
                });
            state.error_message = None;
        }
        _ => {}
    }
}