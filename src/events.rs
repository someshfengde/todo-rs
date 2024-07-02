// events.rs

use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use std::io;


pub fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

pub fn handle_help(f: &mut Frame) {
    let area = f.size();
    let help_area = centered_rect(60, 60, area);
    
    // Render help blocks
    f.render_widget(
        Block::default()
            .title("Help")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        help_area,
    );
    f.render_widget(
        Block::default()
            .title("Press 'q' to quit")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        help_area,
    );
    f.render_widget(
        Block::default()
            .title("Press 'h' to show this help message")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        help_area,
    );
    f.render_widget(
        Block::default()
            .title("Press 'Esc' to clear the help message")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        help_area,
    );
}



// pub fn handle_help(frame: &mut Frame) {
//     // render when pressed h key
//     if event::poll(std::time::Duration::from_millis(50)).unwrap() {
//         if let Event::Key(key) = event::read().unwrap() {
//             if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('h') {
//                 frame.render_widget(
//                     Paragraph::new("Help: Press 'q' to quit, 'h' for help"),
//                     frame.size(),
//                 );
//                 // move back to main block when user press ctrl + c or q key otherwise keep showing help message.
                
//             if let Event::Key(key) = event::read().unwrap() {
//                         if key.kind == event::KeyEventKind::Press
//                             && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('c'))
//                         {
//                             Clear.render(frame.size(), frame);
//                         }
//                     }

//             }
//         }
//     }
// }
