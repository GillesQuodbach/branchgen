use crate::app::editor::{
    backspace_in_selected, delete_in_selected, insert_char_in_selected,
    move_cursor_end_in_selected, move_cursor_home_in_selected,
    move_cursor_left_in_selected, move_cursor_right_in_selected,
    select_next_in_selected, select_prev_in_selected,
};
use crate::app::generator::build_generated_output;
use crate::app::validators::validate_form;
use crate::app::{Action, AppState};
use crate::domain::history::build_history_item;
use crate::storage::json_store::save_history_file;

pub fn update(state: &mut AppState, action: Action) {
    match action {
        Action::Quit => {
            state.should_quit = true;
            state.status = "Bye".to_string();
        }

        Action::Tick => {}

        Action::MoveUp => {
            state.selected_field = state.selected_field.previous();
        }

        Action::MoveDown => {
            state.selected_field = state.selected_field.next();
        }

        Action::MoveLeft => {
            if state.selected_field.is_enum_input() {
                select_prev_in_selected(state);
            } else if state.selected_field.is_text_input() {
                move_cursor_left_in_selected(state);
            }
        }

        Action::MoveRight => {
            if state.selected_field.is_enum_input() {
                select_next_in_selected(state);
            } else if state.selected_field.is_text_input() {
                move_cursor_right_in_selected(state);
            }
        }

        Action::Home => {
            if state.selected_field.is_text_input() {
                move_cursor_home_in_selected(state);
            }
        }

        Action::End => {
            if state.selected_field.is_text_input() {
                move_cursor_end_in_selected(state);
            }
        }

        Action::Backspace => {
            if state.selected_field.is_text_input() {
                backspace_in_selected(state);
            }
        }

        Action::Delete => {
            if state.selected_field.is_text_input() {
                delete_in_selected(state);
            }
        }

        Action::InputCharacter(c) => {
            if state.selected_field.is_text_input() {
                insert_char_in_selected(state, c);
            }
        }

        Action::Enter => {
            if state.selected_field.is_validate() {
                validate_and_generate(state);
            }
        }
    }
}

fn validate_and_generate(state: &mut AppState) {
    match validate_form(&state.work_item_input) {
        Ok(()) => match build_generated_output(&state.work_item_input, &state.team_name) {
            Ok(output) => {
                let history_item =
                    build_history_item(&state.work_item_input, &output, &state.team_name);

                state.generated_output = Some(output);
                state.error_message = None;
                state.status = "Validation successful".to_string();

                state.history.push_item(history_item);

                if let Err(err) = save_history_file(&state.history_file_path, &state.history) {
                    state.error_message = Some(err);
                    state.status = "Failed to save history file".to_string();
                }
            }
            Err(err) => {
                state.generated_output = None;
                state.error_message = Some(err);
                state.status = "Validation failed".to_string();
            }
        },

        Err(err) => {
            state.generated_output = None;
            state.error_message = Some(err);
            state.status = "Validation failed".to_string();
        }
    }
}