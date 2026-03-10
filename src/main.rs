use std::io;

fn main() -> io::Result<()> {
    app::run()
}

mod app;
mod domain;
mod ui;
mod storage;
mod error;
mod config;