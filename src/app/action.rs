use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    Tick,
    MoveUp,
    MoveDown,
    Enter,
    ExitEdition,
    SubmitEdition,
    Backspace,
    InputCharacter(char),
}

pub fn map_key_to_action(code: KeyCode) -> Action {
    match code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Up => Action::MoveUp,
        KeyCode::Down => Action::MoveDown,
        KeyCode::Enter => Action::Enter,
        KeyCode::Esc => Action::ExitEdition,
        KeyCode::Backspace => Action::Backspace,
        KeyCode::Char(c) => Action::InputCharacter(c),
        _ => Action::Tick,
    }
}