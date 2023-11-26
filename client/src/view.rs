use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs};
use ratatui::Frame;

use crate::model::model::ActiveTab;
use crate::{InputMode, Model};

pub fn view(frame: &mut Frame<'_>, app: &Model) {
    // Tabs
    let titles = vec!["Chat", "Logs"];

    let tabs = Tabs::new(titles)
        .select(app.active_tab.get_idx())
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        );

    // Main layout with space for tabs at the top
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(95)])
        .split(frame.size());

    // Render the tabs
    frame.render_widget(tabs, main_layout[0]);

    match app.active_tab {
        ActiveTab::Chat => render_chat_view(frame, app, main_layout[1]),
        ActiveTab::Logs => render_logs_view(frame, app, main_layout[1]),
    }
}

fn render_chat_view(frame: &mut Frame<'_>, app: &Model, area: Rect) {
    // Chat content layout
    let chat_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),   // chat content
            Constraint::Length(3), // user input
            Constraint::Length(1), // bottom bar with keybindings and FPS counter
        ])
        .split(area);

    // Render chat content with a border

    // messages
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            let content = vec![Line::from(Span::raw(m.to_string()))];
            ListItem::new(content)
        })
        .collect();

    let chat_area = chat_layout[0];

    let chat_content = List::new(messages).block(Block::default().borders(Borders::ALL));

    frame.render_widget(chat_content, chat_area);

    // Render user input with a border
    let user_input_area = chat_layout[1];
    let width = chat_layout[0].width.max(3) - 3;
    let scroll = app.input.visual_scroll(width as usize);
    let input_block =
        Block::default()
            .borders(Borders::ALL)
            .style(if app.input_mode == InputMode::Editing {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            });
    frame.render_widget(input_block.clone(), user_input_area);
    let inner_input_area = input_block.inner(user_input_area);
    let user_input = Paragraph::new(app.input.value())
        .scroll((0, scroll as u16))
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Green),
        });
    frame.render_widget(user_input, inner_input_area);

    // Set cursor position if in editing mode
    if let InputMode::Editing = app.input_mode {
        frame.set_cursor(
            inner_input_area.x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16,
            inner_input_area.y,
        );
    }

    // Bottom bar layout for keybindings and FPS counter
    let bottom_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chat_layout[2]);

    // Keybindings
    let tab_keybindings = match app.active_tab {
        ActiveTab::Chat => "tab: switch to logs",
        ActiveTab::Logs => "tab: switch to chat",
    };
    let keybindings = match app.input_mode {
        InputMode::Normal => format!("q: quit | e: edit | {tab_keybindings}"),
        InputMode::Editing => format!("q: quit | esc: stop editing | {tab_keybindings}"),
    };
    frame.render_widget(
        Paragraph::new(keybindings)
            .alignment(Alignment::Left)
            .cyan()
            .bold(),
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
        InputMode::Normal => "Normal",
        InputMode::Editing => "Editing",
    };
    frame.render_widget(
        Paragraph::new(mode_text)
            .alignment(Alignment::Right)
            .style(Style::default().add_modifier(Modifier::BOLD)),
        bottom_bar_layout[1],
    );
}

fn render_logs_view(frame: &mut Frame<'_>, app: &Model, area: Rect) {
    frame.render_widget(
        Paragraph::new("Logs")
            .alignment(Alignment::Left)
            .bold()
            .style(Style::default().fg(Color::Green)),
        area,
    );
}
