#[derive(Debug, Default)]
pub struct AppState {
    pub counter: u8,
    pub should_quit: bool,
    pub status: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            counter: 0,
            should_quit: false,
            status: "Ready".to_string(),
        }
    }
}