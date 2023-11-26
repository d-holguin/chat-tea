use crossterm::event::{
    Event,
    KeyCode::{self, Char},
};
use tui_input::backend::crossterm::EventHandler;

use crate::{model::model::ActiveTab, InputMode, Message, Model};

pub fn update(app: &mut Model, message: Message) {
    match message {
        Message::Key(key) => match app.input_mode {
            InputMode::Normal => match key.code {
                Char('q') => app.message_tx.send(Message::Quit).unwrap(),
                Char('e') => app.input_mode = InputMode::Editing,
                KeyCode::Tab => {
                    app.active_tab = match app.active_tab {
                        ActiveTab::Chat => ActiveTab::Logs,
                        ActiveTab::Logs => ActiveTab::Chat,
                    }
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    let msg = app.input.value().to_string();
                    app.message_tx
                        .send(Message::SendNetworkMessage(msg))
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
        Message::ReceivedNetworkMessage(msg) => {
            app.messages.push(msg);
        }
        Message::SendNetworkMessage(msg) => {
            app.network_manager.send_message(msg);
        }
        _ => {}
    }
}
