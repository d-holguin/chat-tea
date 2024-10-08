use ratatui::widgets::ListItem;
use tui_input::Input;

use crate::{FpsCounter, Message, NetworkManager, Tui};

#[derive(PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
}
#[derive(PartialEq, Eq)]
pub enum ActiveTab {
    Chat,
    Logs,
}

impl ActiveTab {
    pub fn get_idx(&self) -> usize {
        match self {
            ActiveTab::Chat => 0,
            ActiveTab::Logs => 1,
        }
    }
}

// Model state
pub struct Model<'a> {
    pub message_tx: tokio::sync::mpsc::UnboundedSender<Message>,
    pub fps_counter: FpsCounter,
    pub input: Input,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub network_manager: NetworkManager,
    pub active_tab: ActiveTab,
    pub logs: Vec<ListItem<'a>>,
    pub is_user_registered: bool,
}

impl<'a> Model<'a> {
    pub fn new(tui: &Tui, network_manager: NetworkManager) -> Self {
        Self {
            message_tx: tui.event_tx.clone(),
            fps_counter: FpsCounter::new(),
            input: Input::default(),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
            network_manager,
            active_tab: ActiveTab::Chat,
            logs: Vec::new(),
            is_user_registered: false,
        }
    }
}
