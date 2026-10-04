use std::time::Instant;
use chrono::Local;
use indexmap::IndexMap;
use crate::config::FieldType;
use crate::generator::generate_result;
use crate::git::{checkout_branch, create_branch};
use crate::state::{AppState, Step};
use crate::storage::{load_history, save_history, save_persistent, History};

pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Enter,
    Backspace,
    Delete,
    ChangeStep(Step),
    Generate,
    CreateBranch,
    InputCharacter(char),
    Paste(String),
    None,
    NextTab,
    PrevTab,
    HistoryLoaded(usize),
    CopyLineFromResults,
    CopyLineFromHistory,
    ResetForm,
    CheckoutFromHistory,
    CreateBranchFromHistory,
}

fn compute_history_total(len: usize) -> usize {
    match len {
        0 => 0,
        _ => len * 5 - 2,
    }
}

pub fn update(state: &mut AppState, action: Action) {
    match action {
        Action::Quit => state.should_quit = true,

        Action::MoveUp => match state.step {
            Step::History => {
                if state.history_selected_line > 0 {
                    state.history_selected_line -= 1;
                }
            }
            Step::ShowResults => {
                if state.result_selected_line > 0 {
                    state.result_selected_line -= 1;
                }
            }
            _ => {
                if state.form.selected_field > 0 {
                    state.form.selected_field -= 1;
                    state.form.cursor_position = 0;
                    sync_select_position(state);
                }
            }
        },

        Action::MoveDown => match state.step {
            Step::History => {
                if state.history_selected_line < state.history_scroll_limitation {
                    state.history_selected_line += 1;
                }
            }
            Step::ShowResults => {
                if state.result_selected_line < 2 {
                    state.result_selected_line += 1;
                }
            }
            _ => {
                let nb_fields = state.config.fields.len();
                if state.form.selected_field < nb_fields {
                    state.form.selected_field += 1;
                    state.form.cursor_position = 0;
                    sync_select_position(state);
                }
            }
        },

        Action::MoveLeft => {
            if state.form.selected_field < state.config.fields.len() {
                let field_type = state.config.fields[state.form.selected_field].field_type.clone();
                match field_type {
                    FieldType::Select => move_select(state, -1),
                    FieldType::Text | FieldType::Number => {
                        state.form.cursor_position = state.form.cursor_position.saturating_sub(1);
                    }
                }
            }
        }

        Action::MoveRight => {
            if state.form.selected_field < state.config.fields.len() {
                let (field_type, key) = {
                    let field = &state.config.fields[state.form.selected_field];
                    (field.field_type.clone(), field.key.clone())
                };
                match field_type {
                    FieldType::Select => move_select(state, 1),
                    FieldType::Text | FieldType::Number => {
                        let len = state.form.user_inputs
                            .get(&key)
                            .map(|value| value.chars().count())
                            .unwrap_or(0);
                        state.form.cursor_position =
                            (state.form.cursor_position + 1).min(len);
                    }
                }
            }
        }

        Action::InputCharacter(character) => insert_text(state, &character.to_string()),

        Action::Paste(text) => insert_text(state, &text),

        Action::Backspace => {
            if let Some(field) = current_field(state) {
                match field.field_type {
                    FieldType::Select => {}
                    FieldType::Text | FieldType::Number => {
                        let key = field.key.clone();
                        let persistent = field.persistent;
                        if let Some(value) = state.form.user_inputs.get_mut(&key) {
                            if state.form.cursor_position > 0 {
                                if let Some(byte_index) =
                                    char_byte_index(value, state.form.cursor_position - 1)
                                {
                                    value.remove(byte_index);
                                    state.form.cursor_position -= 1;
                                }
                            }
                        }
                        if persistent {
                            save_persistent_fields(state);
                        }
                    }
                }
            }
        }

        Action::Delete => {
            if let Some(field) = current_field(state) {
                match field.field_type {
                    FieldType::Select => {}
                    FieldType::Text | FieldType::Number => {
                        let key = field.key.clone();
                        let persistent = field.persistent;
                        if let Some(value) = state.form.user_inputs.get_mut(&key) {
                            if let Some(byte_index) =
                                char_byte_index(value, state.form.cursor_position)
                            {
                                value.remove(byte_index);
                            }
                        }
                        if persistent {
                            save_persistent_fields(state);
                        }
                    }
                }
            }
        }

        Action::Generate => {
            let result = generate_result(&state.form, &state.config.formats, &state.config.fields);
            state.result = Some(result);
        }

        Action::CreateBranch => {
            if let Some(result) = &state.result {
                match create_branch(&result.branch) {
                    Ok(_) => set_message(state, format!("✓ Branch '{}' created", result.branch)),
                    Err(e) => set_message(state, format!("✗ Error: {}", e)),
                }
            }
        }

        Action::ChangeStep(step) => state.step = step,

        Action::Enter => match state.step {
            Step::FillFields => {
                let last_field = state.config.fields.len();
                if state.form.selected_field == last_field {
                    let missing: Vec<String> = state.config.fields
                        .iter()
                        .filter(|f| f.required)
                        .filter(|f| {
                            state.form.user_inputs
                                .get(&f.key)
                                .map(|v| v.trim().is_empty())
                                .unwrap_or(true)
                        })
                        .map(|f| f.label.clone())
                        .collect();

                    if missing.is_empty() {
                        state.form_error = None;
                        let result = generate_result(
                            &state.form,
                            &state.config.formats,
                            &state.config.fields,
                        );
                        let date = Local::now().format("%d-%m-%Y").to_string();
                        let entry = History {
                            date,
                            branch: result.branch.clone(),
                            commit: result.commit.clone(),
                            pr_title: result.pr_title.clone(),
                        };

                        match save_history(&entry) {
                            Ok(_) => {
                                let history = load_history().unwrap_or_default();
                                state.history_scroll_limitation = compute_history_total(history.len());
                            }
                            Err(e) => {
                                set_message(state, format!("✗ History save error: {}", e));
                                state.history_scroll_limitation =
                                    compute_history_total(load_history().unwrap_or_default().len());
                            }
                        }

                        state.history_scroll = 0;
                        state.result = Some(result);
                        state.step = Step::ShowResults;
                    } else {
                        state.form_error = Some(format!("✗ Required: {}", missing.join(", ")));
                        state.git_message_time = Some(Instant::now());
                    }
                } else {
                    state.form.selected_field += 1;
                    state.form.cursor_position = 0;
                    sync_select_position(state);
                }
            }
            _ => {}
        },

        Action::NextTab => {
            state.step = match state.step {
                Step::FillFields => Step::ShowResults,
                Step::ShowResults => Step::History,
                Step::History => Step::FillFields,
            };
            if state.step == Step::History {
                refresh_history_position(state);
            }
        }

        Action::PrevTab => {
            state.step = match state.step {
                Step::FillFields => Step::History,
                Step::History => Step::ShowResults,
                Step::ShowResults => Step::FillFields,
            };
            if state.step == Step::History {
                refresh_history_position(state);
            }
        }

        Action::HistoryLoaded(total) => {
            state.history_scroll_limitation = total;
            state.history_scroll = 0;
            state.history_selected_line = 0;
        }

        Action::CopyLineFromResults => {
            if let Some(result) = &state.result {
                let text = match state.result_selected_line {
                    0 => &result.branch,
                    1 => &result.commit,
                    _ => &result.pr_title,
                };
                let _ = cli_clipboard::set_contents(text.clone());
                set_message(state, format!("✓ Copied: {}", text));
            }
        }

        Action::CopyLineFromHistory => {
            let history = load_history().unwrap_or_default();
            if history.is_empty() {
                return;
            }

            let line_per_entry = 5;
            let entry_index = state.history_selected_line / line_per_entry;
            let line_in_entry = state.history_selected_line % line_per_entry;

            if let Some(entry) = history.get(entry_index) {
                let text = match line_in_entry {
                    0 => entry.date.clone(),
                    1 => entry.branch.clone(),
                    2 => entry.commit.clone(),
                    3 => entry.pr_title.clone(),
                    _ => return,
                };
                let _ = cli_clipboard::set_contents(text.clone());
                set_message(state, format!("✓ Copied: {}", text));
            }
        }

        Action::ResetForm => {
            state.form.selected_field = 0;
            state.form.cursor_position = 0;
            state.form.select_input_position = 0;
            state.form.user_inputs.clear();
            state.form_error = None;

            let persistent_data = crate::storage::load_persistent();

            for field in &state.config.fields {
                if field.field_type == FieldType::Select {
                    if let Some(values) = &field.values {
                        if let Some(first) = values.first() {
                            state.form.user_inputs.insert(field.key.clone(), first.clone());
                        }
                    }
                }
                if field.persistent {
                    if let Some(value) = persistent_data.get(&field.key) {
                        state.form.user_inputs.insert(field.key.clone(), value.clone());
                    }
                }
            }

            set_message(state, "✓ Form reset".to_string());
        }

        Action::CheckoutFromHistory => {
            let history = load_history().unwrap_or_default();
            if history.is_empty() {
                return;
            }

            let lines_per_entry = 5;
            let entry_index = state.history_selected_line / lines_per_entry;
            let line_in_entry = state.history_selected_line % lines_per_entry;

            if line_in_entry == 1 {
                if let Some(entry) = history.get(entry_index) {
                    match checkout_branch(&entry.branch) {
                        Ok(_) => set_message(state, format!("✓ Switch to '{}'", entry.branch)),
                        Err(e) => set_message(state, format!("✗ Error: {}", e)),
                    }
                }
            } else {
                set_message(state, "✗ Select a branch line first".to_string());
            }
        }

        Action::CreateBranchFromHistory => {
            let history = load_history().unwrap_or_default();
            if history.is_empty() {
                return;
            }

            let lines_per_entry = 5;
            let entry_index = state.history_selected_line / lines_per_entry;
            let line_in_entry = state.history_selected_line % lines_per_entry;

            if line_in_entry == 1 {
                if let Some(entry) = history.get(entry_index) {
                    match create_branch(&entry.branch) {
                        Ok(_) => set_message(state, format!("✓ Branch '{}' created", entry.branch)),
                        Err(e) => set_message(state, format!("✗ Error: {}", e)),
                    }
                }
            } else {
                set_message(state, "✗ Select a branch line first".to_string());
            }
        }

        Action::None => {}
    }
}

fn current_field(state: &AppState) -> Option<&crate::config::FieldConfig> {
    state.config.fields.get(state.form.selected_field)
}

fn move_select(state: &mut AppState, direction: isize) {
    let Some((key, values, persistent)) = current_field(state).and_then(|field| {
        Some((
            field.key.clone(),
            field.values.clone()?,
            field.persistent,
        ))
    }) else {
        return;
    };

    if values.is_empty() {
        return;
    }

    let len = values.len();
    let current = state.form.select_input_position.min(len - 1);
    let next = if direction < 0 {
        if current == 0 { len - 1 } else { current - 1 }
    } else {
        (current + 1) % len
    };

    state.form.select_input_position = next;
    state.form.user_inputs.insert(key, values[next].clone());

    if persistent {
        save_persistent_fields(state);
    }
}

fn insert_text(state: &mut AppState, text: &str) {
    let (field_type, key, persistent) = {
        let Some(field) = current_field(state) else {
            return;
        };
        (field.field_type.clone(), field.key.clone(), field.persistent)
    };

    match field_type {
        FieldType::Select => return,
        FieldType::Number => {
            let filtered: String = text.chars().filter(char::is_ascii_digit).collect();
            insert_at_cursor(state, &key, &filtered);
        }
        FieldType::Text => insert_at_cursor(state, &key, text),
    }

    if persistent {
        save_persistent_fields(state);
    }
}

fn insert_at_cursor(state: &mut AppState, key: &str, text: &str) {
    if text.is_empty() {
        return;
    }

    let cursor = state.form.cursor_position;
    let value = state.form.user_inputs.entry(key.to_string()).or_default();
    let byte_index = char_byte_index(value, cursor).unwrap_or(value.len());
    value.insert_str(byte_index, text);
    state.form.cursor_position += text.chars().count();
}

fn char_byte_index(value: &str, char_index: usize) -> Option<usize> {
    if char_index == value.chars().count() {
        Some(value.len())
    } else {
        value.char_indices().nth(char_index).map(|(index, _)| index)
    }
}

fn save_persistent_fields(state: &AppState) {
    let persistent_data: IndexMap<String, String> = state.config.fields
        .iter()
        .filter(|f| f.persistent)
        .filter_map(|f| {
            state.form.user_inputs.get(&f.key)
                .map(|v| (f.key.clone(), v.clone()))
        })
        .collect();
    let _ = save_persistent(&persistent_data);
}

fn sync_select_position(state: &mut AppState) {
    let Some((field_type, key, values)) = state.config.fields
        .get(state.form.selected_field)
        .map(|field| (field.field_type, field.key.clone(), field.values.clone()))
    else {
        return;
    };

    if field_type != FieldType::Select {
        state.form.select_input_position = 0;
        return;
    }

    let Some(values) = values else {
        state.form.select_input_position = 0;
        return;
    };
    if values.is_empty() {
        state.form.select_input_position = 0;
        return;
    }

    let position = state.form.user_inputs
        .get(&key)
        .and_then(|value| values.iter().position(|item| item == value))
        .unwrap_or(0);

    state.form.select_input_position = position;
}

fn refresh_history_position(state: &mut AppState) {
    let history = load_history().unwrap_or_default();
    state.history_scroll_limitation = compute_history_total(history.len());
    state.history_scroll = 0;
    state.history_selected_line = state
        .history_selected_line
        .min(state.history_scroll_limitation);
}

fn set_message(state: &mut AppState, message: String) {
    state.git_message = Some(message);
    state.git_message_time = Some(Instant::now());
}

#[cfg(test)]
mod tests {
    use super::compute_history_total;

    #[test]
    fn history_max_line_is_correct() {
        assert_eq!(compute_history_total(0), 0);
        assert_eq!(compute_history_total(1), 3);
        assert_eq!(compute_history_total(2), 8);
    }
}
