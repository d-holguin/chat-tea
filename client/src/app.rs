use std::sync::{Arc, Mutex};

use color_eyre::eyre::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};
use tui_input::Input;

use crate::{update, view, TerminalEvent};

// App state
pub struct App {
    pub event_tx: tokio::sync::mpsc::UnboundedSender<TerminalEvent>,
    pub fps_counter: FpsCounter,
    pub input: Input,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub message_tx: tokio::sync::mpsc::UnboundedSender<String>,
}

impl App {
    pub async fn connect_to_server(
        &mut self,
        addr: &str,
        message_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
        incoming_msg_tx: tokio::sync::mpsc::UnboundedSender<String>,
    ) -> Result<()> {
        let stream = TcpStream::connect(addr).await?;
        tokio::spawn(async move {
            handle_connection(stream, message_rx, incoming_msg_tx).await;
        });
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct FpsCounter {
    pub frame_count: u64,
    pub last_tick: std::time::Instant,
    pub fps: u64,
}

pub async fn run() -> Result<()> {
    // ratatui terminal
    let mut tui = crate::Tui::new(4.0, 30.0)?;

    tui.enter()?;

    let fps_counter = FpsCounter {
        frame_count: 0,
        last_tick: std::time::Instant::now(),
        fps: 0,
    };

    let (message_tx, message_rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    let (incoming_msg_tx, mut incoming_msg_rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    // application state
    let mut app = App {
        input_mode: InputMode::Normal,
        event_tx: tui.event_tx.clone(),
        message_tx: message_tx.clone(),
        fps_counter,
        input: Input::default(),
        messages: Vec::new(),
    };
    app.connect_to_server("localhost:8080", message_rx, incoming_msg_tx)
        .await?;

    let mut should_exit = false;
    loop {
        tokio::select! {
            Some(terminal_event) = tui.next() => {
                match terminal_event {
                    TerminalEvent::Render => {
                        app.fps_counter.frame_count += 1;
                        if app.fps_counter.last_tick.elapsed().as_secs() >= 1 {
                            app.fps_counter.fps = app.fps_counter.frame_count;
                            app.fps_counter.frame_count = 0;
                            app.fps_counter.last_tick = std::time::Instant::now();
                        }

                        // Handle the render event
                        tui.terminal.draw(|f| {
                            view(f, &app);
                        })?;
                    },
                    TerminalEvent::Quit => {
                        should_exit = true;
                    },
                    event => {
                        update(&mut app, event);
                    }
                }
            },
            Some(msg) = incoming_msg_rx.recv() => {
                app.messages.push(msg);
            },
        }
        if should_exit {
            break;
        }
    }

    tui.exit()?;
    Ok(())
}

async fn handle_connection(
    mut stream: TcpStream,
    mut message_rx: UnboundedReceiver<String>,
    incoming_msg_tx: UnboundedSender<String>,
) {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => break, // Connection was closed
                    Ok(_) => {
                        incoming_msg_tx.send(line.clone()).unwrap();
                        line.clear();
                    }
                    Err(_) => {
                        break;
                    }
                }
            },
            message = message_rx.recv() => {
                if let Some(msg) = message {
                    //println!("Sending message: {}", msg);
                    let msg = format!("{msg}\n");
                    writer.write_all(msg.as_bytes()).await.unwrap();
                }
            },
        }
    }
}
