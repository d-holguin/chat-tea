use chrono::Local;
use core::fmt;
use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::ListItem,
};
use std::fmt::Write;
use tracing::{
    field::{Field, Visit},
    Event, Level, Subscriber,
};
use tracing_subscriber::Layer;

use crate::Message;

pub struct TuiLogLayer {
    pub message_tx: tokio::sync::mpsc::UnboundedSender<Message>,
}

impl<S: Subscriber> Layer<S> for TuiLogLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let timestamp = Local::now().format("%H:%M:%S");

        let level = event.metadata().level();
        let (level_str, color) = match *level {
            Level::ERROR => ("ERROR", Color::Red),
            Level::INFO => ("INFO", Color::Green),
            Level::WARN => ("WARN", Color::Yellow),
            _ => ("OTHER", Color::White),
        };

        let mut message = String::new();
        let mut vistor = MessageVisitor::new(&mut message);
        event.record(&mut vistor);

        let log_line = Line::styled(
            format!("{timestamp} [{level_str}] {message}"),
            Style::default().fg(color),
        );

        if let Err(e) = self.message_tx.send(Message::Log(ListItem::new(log_line))) {
            eprintln!("Failed to send log message: {}", e);
        }
    }
}

pub struct MessageVisitor<'a> {
    message: &'a mut String,
    is_message_field: bool,
}

impl<'a> MessageVisitor<'a> {
    pub fn new(string: &'a mut String) -> Self {
        MessageVisitor {
            message: string,
            is_message_field: false,
        }
    }
}

impl<'a> Visit for MessageVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            write!(self.message, "{:?}", value).unwrap();
            self.is_message_field = true;
        }
    }
}
