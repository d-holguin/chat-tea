use color_eyre::eyre::Result;
use crossterm::event::KeyCode::Char;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
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

fn ui(frame: &mut Frame<'_>, app: &App) {
    let total_width = frame.size().width as usize;
    let title = "Chatty";
    let fps_text = format!("FPS: {}", app.fps_counter.fps);

    let spacing = total_width.saturating_sub(title.len() + fps_text.len() + 2); // +2 for some padding
    let full_title = format!("{0}{1: >2$}{3}", title, "", spacing, fps_text);

    frame.render_widget(
        Block::default().borders(Borders::ALL).title(full_title),
        frame.size(),
    );

    // main layout 80/20
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .margin(2)
        .split(frame.size());

    // todo: chat content
    let chat_content = format!("Counter: {}", app.counter);
    frame.render_widget(Paragraph::new(chat_content), main_layout[0]);

    // todo: user input
    let user_input = String::from("User input here");
    frame.render_widget(Paragraph::new(user_input), main_layout[1]);
}

pub async fn run() -> Result<()> {
    // ratatui terminal
    let mut tui = crate::Tui::new(3.0, 60.0)?;

    tui.enter()?;

    let fps_counter = FpsCounter {
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
