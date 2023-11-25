use color_eyre::eyre::Result;
use tui_input::Input;

use crate::{update, view, FpsCounter, Message, NetworkManager, Tui};

#[derive(PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
}
// Model state
pub struct Model {
    pub tui_message_tx: tokio::sync::mpsc::UnboundedSender<Message>,
    pub fps_counter: FpsCounter,
    pub input: Input,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub network_manager: NetworkManager,
}

impl Model {
    pub fn new(tui: &Tui, network_manager: NetworkManager) -> Self {
        Self {
            tui_message_tx: tui.event_tx.clone(),
            fps_counter: FpsCounter::new(),
            input: Input::default(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            network_manager,
        }
    }

    pub async fn start(mut self, mut tui: Tui) -> Result<()> {
        tui.enter()?;
        let mut should_exit = false;
        loop {
            tokio::select! {
                Some(message) = tui.next() => {
                    match message {
                        Message::Render => {
                            // Update FPS counter
                            self.fps_counter.tick();
                            // Handle the render event
                            tui.terminal.draw(|f| {
                                view(f, &self);
                            })?;
                        },
                        Message::Quit => {
                            should_exit = true;
                        },
                        message => {
                            update(&mut self, message);
                        }
                    }
                },
                Some(network_msg) = self.network_manager.get_incoming_messages().recv() => {
                    update(&mut self, Message::ReceivedNetworkMessage(network_msg));
                },
            }
            if should_exit {
                break;
            }
        }

        tui.exit()?;
        Ok(())
    }
}

