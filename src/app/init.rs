use std::io;
use crate::app::AppState;
use crate::config::app_config::{load_or_init_config};
use crate::storage::json_store::{get_history_file_path, load_history_file_from_path};

pub fn init_app_state() -> io::Result<AppState> {

    // ici on cree l'appstate au demarrage de l'app
    // recuperation du chemin du fichier
    let history_file_path = get_history_file_path().map_err(io::Error::other)?;

    // charge  l'historique depuis le fichier
    let history = load_history_file_from_path(&history_file_path).map_err(io::Error::other)?;

    // charge ou initialise la config
    let config = load_or_init_config().map_err(io::Error::other)?;

    // recupere le nom de la team
    let team_name = config.team.clone();

    let mut state = AppState::new(team_name, history, history_file_path);

    state.work_item_input.pi = config.default_pi;
    state.work_item_input.it = config.default_it;


    Ok(state)
}