use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use tokio::sync::mpsc;

pub struct Tui {
    pub terminal: ratatui::Terminal<CrosstermBackend<std::io::Stdout>>,
}

pub struct App {
    pub tui: Tui,
    event_tx: tokio::sync::mpsc::Sender<Event>,
    event_rx: tokio::sync::mpsc::Receiver<Event>,
    update_interval: std::time::Duration,
}

pub enum Event {
    Quit,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let terminal = ratatui::Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
        Ok(Self { terminal })
    }

    pub fn enter(&self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            std::io::stdout(),
            EnterAlternateScreen,
            EnableMouseCapture,
            cursor::Hide
        )?;
        Ok(())
    }

    pub fn exit(&self) -> Result<()> {
        crossterm::execute!(
            std::io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            cursor::Show
        )?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().expect("Failed to exit terminal gracefully");
    }
}

async fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                app.event_tx.send(Event::Quit).await?;
            }
        }
    }
    Ok(())
}

impl App {
    pub fn new() -> Result<Self> {
        let (event_tx, event_rx) = mpsc::channel(100);
        let tui = Tui::new()?;
        let update_interval = std::time::Duration::from_millis(50);

        Ok(Self {
            tui,
            event_tx,
            event_rx,
            update_interval,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.tui.enter()?;
        let mut interval = tokio::time::interval(self.update_interval);
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.tui.terminal.draw(|f| {
                        let size = f.size();
                        let block = ratatui::widgets::Block::default()
                            .title("Hello World!(Press q to quit)")
                            .borders(ratatui::widgets::Borders::ALL);
                        f.render_widget(block, size);
                    })?;
                    update(self).await?;
                }
                Some(event) = self.event_rx.recv() => {
                    match event {
                        Event::Quit => break,
                    }
                }
            }
        }

        self.tui.exit()?;
        Ok(())
    }
}
