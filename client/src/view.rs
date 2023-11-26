use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs};
use ratatui::Frame;

use crate::model::model::ActiveTab;
use crate::{InputMode, Model};

pub fn view(frame: &mut Frame<'_>, model: &Model) {
    // Tabs
    let titles = vec!["Chat", "Logs"];

    let tabs = Tabs::new(titles)
        .select(model.active_tab.get_idx())
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        );

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),          // tabs
            Constraint::Percentage(100), // main content
            Constraint::Min(1),          // bottom bar keybindings, FPS counter, etc.
        ])
        .split(frame.size());

    // Render the tabs
    frame.render_widget(tabs, main_layout[0]);

    match model.active_tab {
        ActiveTab::Chat => render_chat_view(frame, model, main_layout[1]),
        ActiveTab::Logs => render_logs_view(frame, model, main_layout[1]),
    }

    // Bottom bar layout for keybindings and FPS counter
    let bottom_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[2]);

    // Keybindings
    let keybindings = match model.active_tab {
        ActiveTab::Chat => match model.input_mode {
            InputMode::Normal => "q: quit | enter: edit | tab: logs",
            InputMode::Editing => "q: quit | esc: stop editing | tab: logs",
        },
        ActiveTab::Logs => "q: quit | tab: chat",
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
        Paragraph::new(format!("FPS: {}", model.fps_counter.fps))
            .alignment(Alignment::Left)
            .blue(),
        bottom_bar_layout[1],
    );

    // Mode indicator
    let mode_text = match model.input_mode {
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

fn render_chat_view(frame: &mut Frame<'_>, app: &Model, area: Rect) {
    // Chat content layout
    let chat_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(100), // chat content
            Constraint::Min(3),          // user input
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
}

fn render_logs_view(frame: &mut Frame<'_>, model: &Model, area: Rect) {
    let logs = List::new(model.logs.clone()).block(Block::default().borders(Borders::ALL));

    frame.render_widget(logs, area);
}
