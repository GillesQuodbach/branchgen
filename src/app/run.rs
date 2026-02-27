use std::io;

use crate::app::{update::update, Action, AppState};
use crate::ui::{event, terminal, tui};

pub fn run() -> io::Result<()> {
    let mut terminal = terminal::init();
    let mut state = AppState::new();

    // On restaure toujours le terminal à la fin
    let result = run_loop(&mut terminal, &mut state);

    terminal::restore();
    result
}

fn run_loop(terminal: &mut ratatui::DefaultTerminal, state: &mut AppState) -> io::Result<()> {
    while !state.should_quit {
        terminal.draw(|f| tui::render(f, state))?;

        // Lis une action (avec tick)
        let action: Action = event::next_action()?;
        update(state, action);
    }
    Ok(())
}