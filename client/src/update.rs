use crossterm::event::{
    Event,
    KeyCode::{self, Char},
};
use tui_input::backend::crossterm::EventHandler;

use crate::{App, InputMode, TerminalEvent};

pub fn update(app: &mut App, event: TerminalEvent) {
    match event {
        TerminalEvent::Key(key) => match app.input_mode {
            InputMode::Normal => match key.code {
                Char('q') => app.event_tx.send(TerminalEvent::Quit).unwrap(),
                Char('e') => app.input_mode = InputMode::Editing,
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    let message = app.input.to_string();
                    app.event_tx
                        .send(TerminalEvent::SendMessage(message))
                        .unwrap();
                    app.input.reset();
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                }
                _ => {
                    app.input.handle_event(&Event::Key(key));
                }
            },
        },
        TerminalEvent::Network(message) => {
            app.messages.push(message.message);
        }
        _ => {}
    }
}
