use std::io;

use crate::app::{init::init_app_state, update::update, Action, AppState};
use crate::ui::{event, terminal, tui};

pub fn run() -> io::Result<()> {
    let mut terminal = terminal::init();
    let mut state = init_app_state()?;

    let result = run_loop(&mut terminal, &mut state);

    terminal::restore();
    result
}

fn run_loop(terminal: &mut ratatui::DefaultTerminal, state: &mut AppState) -> io::Result<()> {
    while !state.should_quit {
        terminal.draw(|f| tui::render(f, state))?;

        let action: Action = event::next_action()?;
        update(state, action);
    }

    Ok(())
}