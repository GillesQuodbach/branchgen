use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyEventKind};

use crate::app::action::{Action, map_key_to_action};

pub fn next_action() -> io::Result<Action> {
    // Tick : l'UI continue de respirer même sans input
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                return Ok(map_key_to_action(key));
            }
            _ => {}
        }
    }
    Ok(Action::Tick)
}

