use crate::app::{Action, AppState};

pub fn update(state: &mut AppState, action: Action) {
    match action {
        Action::Quit => {
            state.should_quit = true;
            state.status = "Bye".to_string();
        }
        Action::Increment => {
            state.counter = state.counter.saturating_add(1);
            state.status = "Increment".to_string();
        }
        Action::Decrement => {
            state.counter = state.counter.saturating_sub(1);
            state.status = "Decrement".to_string();
        }
        Action::Tick => {
            // Optionnel : animations / horloge / autosave, etc.
        }
    }
}