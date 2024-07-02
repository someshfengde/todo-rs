// main.rs

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};
use std::io::{self, stdout};
mod drawing;
mod events;

use crate::drawing::main_block;
use crate::events::handle_events;

// mod terminal;

fn main() -> io::Result<()> {
    let arg = std::env::args().nth(1).unwrap_or_default();

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(match arg.as_str() {
            "layout" => main_block,
            "styling" => main_block,
            _ => main_block,
        })?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
