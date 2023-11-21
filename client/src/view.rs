use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::{App, InputMode};

pub fn view(frame: &mut Frame<'_>, app: &App) {
    // Main layout: chat content and bottom bar
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(frame.size());

    // Chat content layout
    let chat_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(main_layout[0]);

    // Render chat content with a border
    let chat_area = chat_layout[0];
    let chat_content =
        Paragraph::new("Chat content here...").block(Block::default().borders(Borders::ALL));
    frame.render_widget(chat_content, chat_area);

    // Render user input with a border
    let user_input_area = chat_layout[1];
    let input_block = Block::default().borders(Borders::ALL);
    frame.render_widget(input_block.clone(), user_input_area);
    let inner_input_area = input_block.inner(user_input_area);
    let user_input = Paragraph::new(app.input.value()).style(match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Editing => Style::default().fg(Color::Green),
    });
    frame.render_widget(user_input, inner_input_area);

    // Set cursor position if in editing mode
    if let InputMode::Editing = app.input_mode {
        let cursor_position = app.input.cursor();
        frame.set_cursor(
            inner_input_area.x + cursor_position as u16,
            inner_input_area.y,
        );
    }

    // Bottom bar layout for keybindings and FPS counter
    let bottom_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    // Keybindings
    let keybindings = match app.input_mode {
        InputMode::Normal => "q: quit | e: edit",
        InputMode::Editing => "q: quit | esc: stop editing",
    };
    frame.render_widget(
        Paragraph::new(keybindings)
            .alignment(Alignment::Left)
            .cyan()
            .italic(),
        bottom_bar_layout[0],
    );

    // FPS counter
    frame.render_widget(
        Paragraph::new(format!("FPS: {}", app.fps_counter.fps))
            .alignment(Alignment::Left)
            .blue(),
        bottom_bar_layout[1],
    );

    // Mode indicator
    let mode_text = match app.input_mode {
        InputMode::Normal => "Mode: Normal",
        InputMode::Editing => "Mode: Editing",
    };
    frame.render_widget(
        Paragraph::new(mode_text)
            .alignment(Alignment::Right)
            .style(Style::default().add_modifier(Modifier::ITALIC)),
        bottom_bar_layout[1],
    );
}
