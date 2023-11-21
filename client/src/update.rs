use crossterm::event::{
    Event,
    KeyCode::{self, Char},
};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::{InputMode, App};



pub fn update(app: &mut App, event: crate::Event) {
    if let crate::Event::Key(key) = event {
        match app.input_mode {
            InputMode::Normal => match key.code {
                Char('q') => app.should_quit = true,
                Char('e') => app.input_mode = InputMode::Editing,
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    app.input.reset();
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                }
                _ => {
                    app.input.handle_event(&Event::Key(key));
                }
            },
        }
    }
}
