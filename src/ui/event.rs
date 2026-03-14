use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyEventKind};

use crate::app::action::{Action, map_key_to_action};
use crate::app::input_mode::InputMode;

pub fn next_action(input_mode: InputMode) -> io::Result<Action> {
    // Tick : l'UI continue de respirer même sans input
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                return Ok(map_key_to_action(key.code, input_mode));
            }
            _ => {}
        }
    }
    Ok(Action::Tick)
}