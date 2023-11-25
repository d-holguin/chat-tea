use color_eyre::eyre::Result;
use tokio::{
    net::TcpStream,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};
use tui_input::Input;

use crate::{update, view, FpsCounter, Message, Tui};

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
    pub sending_network_msg_tx: tokio::sync::mpsc::UnboundedSender<String>,
}

impl Model {
    pub fn new(tui: &Tui, sending_message_tx: UnboundedSender<String>) -> Self {
        Self {
            tui_message_tx: tui.event_tx.clone(),
            fps_counter: FpsCounter::new(),
            input: Input::default(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            sending_network_msg_tx: sending_message_tx.clone(),
        }
    }
    pub async fn connect_to_server(
        &mut self,
        addr: &str,
        message_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
        incoming_msg_tx: tokio::sync::mpsc::UnboundedSender<String>,
    ) -> Result<()> {
        let stream = TcpStream::connect(addr).await?;
        tokio::spawn(async move {
            crate::handle_connection::manage_tcp_stream(stream, message_rx, incoming_msg_tx).await;
        });
        Ok(())
    }

    pub async fn start(
        mut self,
        mut tui: Tui,
        sending_message_rx: UnboundedReceiver<String>,
        incoming_msg_tx: UnboundedSender<String>,
        mut incoming_network_msg_rx: UnboundedReceiver<String>,
    ) -> Result<()> {
        self.connect_to_server("localhost:8080", sending_message_rx, incoming_msg_tx)
            .await?;

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
                Some(network_msg) = incoming_network_msg_rx.recv() => {
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

pub async fn run() -> Result<()> {
    let mut tui = crate::Tui::new(4.0, 30.0)?;

    tui.enter()?;

    let (sending_network_msg_tx, sending_network_msg_rx) =
        tokio::sync::mpsc::unbounded_channel::<String>();
    let (incoming_network_msg_tx, incoming_network_msg_rx) =
        tokio::sync::mpsc::unbounded_channel::<String>();

    let app = Model::new(&tui, sending_network_msg_tx.clone());

    app.start(
        tui,
        sending_network_msg_rx,
        incoming_network_msg_tx,
        incoming_network_msg_rx,
    )
    .await?;
    Ok(())
}
