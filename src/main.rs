use std::io;

fn main() -> io::Result<()> {
    // appel run
    app::run()
}

mod app;
mod domain;
mod ui;
mod storage;
mod error;
mod config;