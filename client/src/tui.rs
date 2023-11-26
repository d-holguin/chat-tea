use color_eyre::eyre::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::{
    event::{KeyEvent, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend;

use ratatui::widgets::ListItem;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tracing::error;

#[derive(Clone, Debug)]
pub enum Message {
    Quit,
    Error,
    Tick,
    Render,
    Key(KeyEvent),
    ReceivedNetworkMessage(String),
    SendNetworkMessage(String),
    Log(ListItem<'static>),
}

pub struct Tui {
    pub terminal: ratatui::Terminal<CrosstermBackend<std::io::Stdout>>,
    pub task: JoinHandle<()>,
    pub event_rx: UnboundedReceiver<Message>,
    pub event_tx: UnboundedSender<Message>,
    pub frame_rate: f64,
    pub tick_rate: f64,
}

impl Tui {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let terminal = ratatui::Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
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

    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        self.start();
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        if crossterm::terminal::is_raw_mode_enabled()? {
            self.terminal.flush()?;
            crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
            crossterm::terminal::disable_raw_mode()?;
            self.terminal.show_cursor()?;
        }
        Ok(())
    }

    pub async fn next(&mut self) -> Option<Message> {
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
                                    if let Err(e) = event_tx.send(Message::Key(key)) {
                                        error!("Failed to send key event: {}", e);
                                    }
                                }
                              },
                              _ =>{}

                            }
                          }
                          Some(Err(_e)) => {
                            if let Err(e) = event_tx.send(Message::Error) {
                                error!("Failed to send error event: {}", e);
                            }
                          }
                          None => {},
                        }
                      },
                      _ = tick_delay => {
                        if let Err(e) = event_tx.send(Message::Tick){
                            error!("Failed to send tick event: {}", e);
                        }
                      },
                      _ = render_delay => {
                        if let Err(e) = event_tx.send(Message::Render){
                            error!("Failed to send render event: {}", e);
                      }
                    }
                }
            }
        });
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().expect("Failed to exit terminal");
    }
}
