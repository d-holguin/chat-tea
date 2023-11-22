use color_eyre::eyre::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::{
    event::{KeyEvent, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

#[derive(Clone, Debug)]
pub enum TerminalEvent {
    Quit,
    Error,
    Tick,
    Render,
    Key(KeyEvent),
    Network(NetworkData),
    SendMessage(String),
}
#[derive(Clone, Debug)]
pub struct NetworkData {
    pub message: String,
}

pub struct Tui {
    pub terminal: ratatui::Terminal<CrosstermBackend<std::io::Stderr>>,
    pub task: JoinHandle<()>,
    pub event_rx: UnboundedReceiver<TerminalEvent>,
    pub event_tx: UnboundedSender<TerminalEvent>,
    pub frame_rate: f64,
    pub tick_rate: f64,
}

impl Tui {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let terminal = ratatui::Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let task = tokio::spawn(async {});
        Ok(Self {
            terminal,
            task,
            event_rx,
            event_tx,
            frame_rate,
            tick_rate,
        })
    }

    pub async fn connect_to_server(&mut self, addr: &str) -> Result<()> {
        let stream = TcpStream::connect(addr).await?;
        let event_tx = self.event_tx.clone();
        tokio::spawn(async move {
            handle_connection(stream, event_tx).await;
        });
        Ok(())
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
        self.start();
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        if crossterm::terminal::is_raw_mode_enabled()? {
            self.terminal.flush()?;
            crossterm::execute!(std::io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
            crossterm::terminal::disable_raw_mode()?;
            self.terminal.show_cursor()?;
        }
        Ok(())
    }

    pub async fn next(&mut self) -> Option<TerminalEvent> {
        self.event_rx.recv().await
    }

    pub fn start(&mut self) {
        let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.tick_rate);
        let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);
        let event_tx = self.event_tx.clone();
        self.task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_delay);
            let mut render_interval = tokio::time::interval(render_delay);
            loop {
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                  maybe_event = crossterm_event => {
                    match maybe_event {
                      Some(Ok(evt)) => {
                        match evt {
                          crossterm::event::Event::Key(key) => {
                            if key.kind == KeyEventKind::Press {
                              event_tx.send(TerminalEvent::Key(key)).unwrap();
                            }
                          },
                          _ =>{}

                        }
                      }
                      Some(Err(_)) => {
                        event_tx.send(TerminalEvent::Error).unwrap();
                      }
                      None => {},
                    }
                  },
                  _ = tick_delay => {
                      event_tx.send(TerminalEvent::Tick).unwrap();
                  },
                  _ = render_delay => {
                      event_tx.send(TerminalEvent::Render).unwrap();
                  },
                }
            }
        });
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}

async fn handle_connection(mut stream: TcpStream, event_tx: UnboundedSender<TerminalEvent>) {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => break, // Connection was closed
                    Ok(_) => {
                        event_tx.send(TerminalEvent::Network(NetworkData { message: line.clone() })).unwrap();
                        line.clear();
                    }
                    Err(_) => {
                        event_tx.send(TerminalEvent::Error).unwrap();
                        break;
                    }
                }

            }
      
            // todo use writer to send messages

        }
    }
}
