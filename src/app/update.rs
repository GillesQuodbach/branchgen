use crate::app::{Action, AppState};
use crate::app::input_mode::InputMode;
use crate::domain::types::GeneratedOutput;

fn build_generated_output(state: &AppState) -> Result<GeneratedOutput, String> {
    let branch_name = state.work_item_input.branch_name(&state.team_name)?;
    let checkout_cmd = GeneratedOutput::format_checkout_cmd(&branch_name);
    let commit_msg = state.work_item_input.commit_name(&state.team_name)?;
    let pr_title = state.work_item_input.pr_name(&state.team_name)?;

    Ok(GeneratedOutput::new(
        checkout_cmd,
        branch_name,
        commit_msg,
        pr_title,
    ))
}
pub fn update(state: &mut AppState, action: Action) {
    match state.input_mode {
        InputMode::Navigation => {
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
                    if state.selected_field.is_selectable() {
                        state.select_prev_in_selected()
                    }
                }
                Action::MoveRight => {
                    if state.selected_field.is_selectable(){
                        state.select_next_in_selected()
                    }
                }
                Action::Enter => {
                    if state.selected_field.is_validate() {
                        match state.validate_form() {
                            Ok(()) => {
                                match build_generated_output(state) {
                                    Ok(output) => {
                                        state.generated_output = Some(output);
                                        state.error_message = None;
                                        state.status = "Validation successful".to_string();
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

                    else if state.selected_field.is_editable() {
                        state.input_mode = InputMode::Edition;
                        state.status = format!("Editing {:?}", state.selected_field);
                    }
                }
                Action::ExitEdition | Action::Backspace | Action::InputCharacter(_) => {}
                _ => {}
            }
        }
        InputMode::Edition => {
            match action {
                 Action::Quit => {
                    state.should_quit = true;
                    state.status = "Bye".to_string();
                 }
                Action::Tick => {}
                Action::Enter => {
                    state.input_mode = InputMode::Navigation;
                    state.status = "Navigation".to_string();

                    match state.validate_current_field() {
                        Ok(()) => {
                            state.error_message = None;
                        }
                        Err(err) => {
                            state.error_message = Some(err);
                        }
                    }
                }
                Action::ExitEdition => {
                    state.input_mode = InputMode::Navigation;
                    state.status = "Navigation".to_string();
                }
                Action::Backspace => {
                    state.backspace_in_selected()
                }
                Action::InputCharacter(c) => {
                    state.insert_char_in_selected(c)
                }
                Action::MoveUp | Action::MoveDown => {}

                _ => {}
            }
        }

    }
}