use crate::app::{Action, AppState};

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
    }
}