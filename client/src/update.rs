use crossterm::event::{
    Event,
    KeyCode::{self, Char},
};
use tui_input::backend::crossterm::EventHandler;

use crate::{App, InputMode, Message};

pub fn update(app: &mut App, message: Message) {
    match message {
        Message::Key(key) => match app.input_mode {
            InputMode::Normal => match key.code {
                Char('q') => app.tui_event_tx.send(Message::Quit).unwrap(),
                Char('e') => app.input_mode = InputMode::Editing,
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    let msg = app.input.value().to_string();
                    app.sending_message_tx.send(msg).unwrap();
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
        Message::ReceivedNetworkMessage(msg) => {
            app.messages.push(msg);
        }
        _ => {}
    }
}
