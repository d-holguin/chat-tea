use color_eyre::eyre::Result;
use crossterm::event::KeyCode::Char;
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::Event;

// App state
pub struct App {
    counter: i64,
    should_quit: bool,
    fps_counter: FpsCounter,
}

pub struct FpsCounter {
    pub frame_count: u64,
    pub last_tick: std::time::Instant,
    pub fps: u64,
}

fn update(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match key.code {
            Char('j') => app.counter += 1,
            Char('k') => app.counter -= 1,
            Char('q') => app.should_quit = true,
            _ => {}
        }
    }
}

fn ui(f: &mut Frame<'_>, app: &App) {
    let content = format!("Counter: {}\nFPS: {}", app.counter, app.fps_counter.fps);
    f.render_widget(Paragraph::new(content).white().on_cyan(), f.size());
}

pub async fn run() -> Result<()> {
    // ratatui terminal
    let mut tui = crate::Tui::new(4.0, 30.0)?;

    tui.enter()?;

    let mut fps_counter = FpsCounter {
        frame_count: 0,
        last_tick: std::time::Instant::now(),
        fps: 0,
    };

    // application state
    let mut app = App {
        counter: 0,
        should_quit: false,
        fps_counter,
    };

    loop {
        match tui.next().await {
            Some(Event::Render) => {
                app.fps_counter.frame_count += 1;
                if app.fps_counter.last_tick.elapsed().as_secs() >= 1 {
                    app.fps_counter.fps = app.fps_counter.frame_count;
                    app.fps_counter.frame_count = 0;
                    app.fps_counter.last_tick = std::time::Instant::now();
                }

                // Handle the render event
                tui.terminal.draw(|f| {
                    ui(f, &app);
                })?;
            }
            Some(Event::Quit) => {
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
