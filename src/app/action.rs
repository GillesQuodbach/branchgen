use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    Tick,
    MoveUp,
    MoveDown,
}

pub fn map_key_to_action(code: KeyCode) -> Action {
    match code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Up => Action::MoveUp,
        KeyCode::Down => Action::MoveDown,
        _ => Action::Tick,
    }
}