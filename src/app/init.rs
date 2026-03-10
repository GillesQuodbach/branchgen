use std::io;
use crate::app::AppState;
use crate::config::app_config::load_or_init_config;

pub fn init_app_state() -> io::Result<AppState> {

    let config = load_or_init_config().map_err(io::Error::other)?;

    let team_name = config.team.clone();
    let history = Vec::new();

    let mut state = AppState::new(team_name, history);

    state.work_item_input.pi = config.default_pi;
    state.work_item_input.it = config.default_it;


    Ok(state)
}