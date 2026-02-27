use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::Action;

pub fn next_action() -> io::Result<Action> {
    // Tick : l'UI continue de respirer même sans input
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                return Ok(map_key_to_action(key.code));
            }
            _ => {}
        }
    }
    Ok(Action::Tick)
}

fn map_key_to_action(code: KeyCode) -> Action {
    match code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Left => Action::Decrement,
        KeyCode::Right => Action::Increment,
        _ => Action::Tick,
    }
}