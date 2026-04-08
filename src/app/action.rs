use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    Tick,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Enter,
    Backspace,
    Delete,
    Home,
    End,
    InputCharacter(char),
}

pub fn map_key_to_action(key: KeyEvent) -> Action {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), modifiers) if modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        (KeyCode::Char('c'), modifiers) if modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,

        (KeyCode::Up, _) => Action::MoveUp,
        (KeyCode::Down, _) => Action::MoveDown,
        (KeyCode::Left, _) => Action::MoveLeft,
        (KeyCode::Right, _) => Action::MoveRight,
        (KeyCode::Enter, _) => Action::Enter,
        (KeyCode::Backspace, _) => Action::Backspace,
        (KeyCode::Delete, _) => Action::Delete,
        (KeyCode::Home, _) => Action::Home,
        (KeyCode::End, _) => Action::End,

        (KeyCode::Char(c), modifiers)
        if !modifiers.contains(KeyModifiers::CONTROL)
            && !modifiers.contains(KeyModifiers::ALT) =>
            {
                Action::InputCharacter(c)
            }

        _ => Action::Tick,
    }
}