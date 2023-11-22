use color_eyre::eyre::Result;
use tui_input::Input;

use crate::{update, view, TerminalEvent};

// App state
pub struct App {
    pub event_tx: tokio::sync::mpsc::UnboundedSender<TerminalEvent>,
    pub fps_counter: FpsCounter,
    pub input: Input,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
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
    tui.connect_to_server("localhost:8080").await?;

    let fps_counter = FpsCounter {
        frame_count: 0,
        last_tick: std::time::Instant::now(),
        fps: 0,
    };

    // application state
    let mut app = App {
        input_mode: InputMode::Normal,
        event_tx: tui.event_tx.clone(),
        fps_counter,
        input: Input::default(),
        messages: Vec::new(),
    };
    app.messages.push("This is a test message".into());
    app.messages.push("This is another test message".into());

    loop {
        match tui.next().await {
            Some(TerminalEvent::Render) => {
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
            }
            Some(TerminalEvent::Quit) => {
                break;
            }
            Some(event) => {
                update(&mut app, event);
            }
            None => {}
        }
    }
    tui.exit()?;
    Ok(())
}
