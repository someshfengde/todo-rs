use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

use ratatui::Frame;

use crate::events::handle_help;
pub fn layout() {
    // Implement layout drawing logic
    ()
}

pub fn styling() {
    // Implement styling drawing logic
    ()
}

pub fn main_block(frame: &mut Frame,) {
    let mut show_help = false;
    // rendering the main bock title.
    frame.render_widget(
        Block::bordered()
            .border_set(symbols::border::DOUBLE)
            .title_top(Line::from(" AI TODO.rs ").centered())
            .title_style(Style::default().fg(Color::Red).bg(Color::Black))
            .border_style(Style::default().fg(Color::Green).bg(Color::Black)),
        frame.size(),
    );
    // take input for hte help message if the user presses 'h' render help message until esc or q is pressed. 
    if event::poll(std::time::Duration::from_millis(50)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('h') {
                show_help = true;
            }
        }
    }
    if show_help {
        handle_help(frame);
    }

}
