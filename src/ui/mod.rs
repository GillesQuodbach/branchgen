use crate::state::{AppState, Step};
use ratatui::Frame;

pub mod form;
pub mod history;
pub mod input;
pub mod layout;
pub mod result;

pub fn render(frame: &mut Frame, state: &AppState) {
    match state.step {
        Step::FillFields => form::render_fields(frame, state),
        Step::ShowResults => result::render_results(frame, state),
        Step::History => history::render_history(frame, state),
    }
}
