use crossterm::event::KeyCode;
use crate::app::input_mode::InputMode;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    Tick,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Enter,
    ExitEdition,
    Backspace,
    InputCharacter(char),
}

pub fn map_key_to_action(code: KeyCode, input_mode: InputMode) -> Action {
    match code {
        KeyCode::Char('q') if matches!(input_mode, InputMode::Navigation) => Action::Quit,
        KeyCode::Up => Action::MoveUp,
        KeyCode::Down => Action::MoveDown,
        KeyCode::Left => Action::MoveLeft,
        KeyCode::Right => Action::MoveRight,
        KeyCode::Enter => Action::Enter,
        KeyCode::Esc => Action::ExitEdition,
        KeyCode::Backspace => Action::Backspace,
        KeyCode::Char(c) => Action::InputCharacter(c),
        _ => Action::Tick,
    }
}