use color_eyre::eyre::Result;
use crossterm::event::{
    Event,
    KeyCode::{self, Char},
};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::view;

// App state
pub struct App {
    pub should_quit: bool,
    pub fps_counter: FpsCounter,
    pub input: Input,
    pub input_mode: InputMode,
}

pub enum InputMode {
    Normal,
    Editing,
}

pub struct FpsCounter {
    pub frame_count: u64,
    pub last_tick: std::time::Instant,
    pub fps: u64,
}

fn update(app: &mut App, event: crate::Event) {
    if let crate::Event::Key(key) = event {
        match app.input_mode {
            InputMode::Normal => match key.code {
                Char('q') => app.should_quit = true,
                Char('e') => app.input_mode = InputMode::Editing,
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    app.input.reset();
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                }
                _ => {
                    app.input.handle_event(&Event::Key(key));
                }
            },
        }
    }
}

pub async fn run() -> Result<()> {
    // ratatui terminal
    let mut tui = crate::Tui::new(4.0, 144.0)?;

    tui.enter()?;

    let fps_counter = FpsCounter {
        frame_count: 0,
        last_tick: std::time::Instant::now(),
        fps: 0,
    };

    // application state
    let mut app = App {
        input_mode: InputMode::Normal,
        should_quit: false,
        fps_counter,
        input: Input::default(),
    };

    loop {
        match tui.next().await {
            Some(crate::Event::Render) => {
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
            Some(crate::Event::Quit) => {
                // Handle the quit event
                break;
            }
            Some(event) => {
                // Handle other events
                update(&mut app, event);

                if app.should_quit {
                    break;
                }
            }
            None => {}
        }
    }

    tui.exit()?;

    Ok(())
}
