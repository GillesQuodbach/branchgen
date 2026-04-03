use std::io;

use crate::app::{init::init_app_state, update::update, Action, AppState};
use crate::ui::{event, terminal, tui};

pub fn run() -> io::Result<()> {
    // run est le moteur de la boucle
    // initialise le terminal
    let mut terminal = terminal::init();
    // créé le state initial
    let mut state = init_app_state()?;

    // lance la boucle principale
    let result = run_loop(&mut terminal, &mut state);

    // reset le termianl a la fin de la boucle
    terminal::restore();
    result
}

fn run_loop(terminal: &mut ratatui::DefaultTerminal, state: &mut AppState) -> io::Result<()> {
    while !state.should_quit {
        terminal.draw(|f| tui::render(f, state))?;

        // Lis une action (avec tick)
        let action: Action = event::next_action(state.input_mode)?;
        update(state, action);
    }
    Ok(())
}