
use crate::app::{Action, AppState};
use crate::app::generator::build_generated_output;
use crate::app::validators::{validate_current_field, validate_form};
use crate::app::editor::{insert_char_in_selected, backspace_in_selected, select_next_in_selected, select_prev_in_selected};
use crate::app::input_mode::InputMode;
use crate::domain::history::{build_history_item};
use crate::storage::json_store::{save_history_file};

pub fn update(state: &mut AppState, action: Action) {
    match action {
                Action::Quit => {
                    state.should_quit = true;
                    state.status = "Bye".to_string();
                }
                Action::Tick => {
                    // Optionnel : animations / horloge / autosave, etc.
                }
                Action::MoveUp => {
                    state.selected_field = state.selected_field.previous();
                }
                Action::MoveDown => {
                    state.selected_field = state.selected_field.next();
                }
                Action::MoveLeft => {
                    if state.selected_field.is_enum_input() {
                        select_prev_in_selected(state)
                    } else if state.selected_field.is_text_input() {
                        // TODO deplacer cursor
                    }
                }
                Action::MoveRight => {
                    if state.selected_field.is_enum_input(){
                        select_next_in_selected(state)
                    } else if state.selected_field.is_text_input(){
                        // TODO deplacer cursor
                    }
                }
                Action::Enter => {
                    if state.selected_field.is_validate() {
                        match validate_form(&state.work_item_input) {
                            Ok(()) => {
                                match build_generated_output(&state.work_item_input, &state.team_name) {

                                    Ok(output) => {

                                        let history_item =build_history_item(&state.work_item_input, &output, &state.team_name);
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
                                }
                            }
                            Err(err) => {
                                state.generated_output = None;
                                state.error_message = Some(err);
                                state.status = "Validation failed".to_string();
                            }
                        }
                    }
                }
             Action::Backspace | Action::InputCharacter(_) => {}
                _ => {}
            }
        }