use crossterm::event::{
    Event,
    KeyCode::{self, Char},
};
use tracing::error;
use tui_input::backend::crossterm::EventHandler;

use crate::{model::model::ActiveTab, InputMode, Message, Model};

pub fn update(model: &mut Model, message: Message) {
    match message {
        Message::Key(key) => match model.input_mode {
            InputMode::Normal => match key.code {
                Char('q') => {
                    if let Err(e) = model.message_tx.send(Message::Quit) {
                        error!("Failed to send quit message: {}", e)
                    }
                }
                KeyCode::Enter => {
                    if model.active_tab == ActiveTab::Chat || !model.is_user_registered {
                        model.input_mode = InputMode::Editing;
                    }
                }
                KeyCode::Tab => {
                    model.active_tab = match model.active_tab {
                        ActiveTab::Chat => ActiveTab::Logs,
                        ActiveTab::Logs => ActiveTab::Chat,
                    }
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    if model.is_user_registered {
                        let msg = model.input.value().to_string();
                        if let Err(e) = model.message_tx.send(Message::SendNetworkMessage(msg)) {
                            error!("Failed to send message: {}", e)
                        }
                        model.input.reset();
                    } else {
                        let username = format!("username:{}", model.input.value().to_string());
                        if let Err(e) = model.message_tx.send(Message::RegisterUser(username)) {
                            error!("Failed to send register message: {}", e)
                        }
                    }
                }
                KeyCode::Esc => {
                    model.input_mode = InputMode::Normal;
                }
                _ => {
                    model.input.handle_event(&Event::Key(key));
                }
            },
        },
        Message::RegisterUser(username) => {
            model.network_manager.send_message(username);
            model.is_user_registered = true;
            model.input.reset();
        }
        Message::ReceivedNetworkMessage(msg) => {
            model.messages.push(msg);
        }
        Message::SendNetworkMessage(msg) => {
            model.network_manager.send_message(msg);
        }
        Message::Log(msg) => {
            model.logs.push(msg);
        }
        _ => {}
    }
}
