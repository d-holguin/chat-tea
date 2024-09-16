# ChatTea

![Usage Example](https://github.com/d-holguin/chat_tea/blob/main/example_images/chat_tea_demo.gif)

ChatTea is primarily a learning project for Rust, Tokio, and concurrent programming. 

This project is a Rust-based, asynchronous chat application with a Terminal User Interface (TUI), leveraging Tokio for efficient async operations and [ratatui](https://github.com/ratatui-org/ratatui) for the Terminal User Interface. 

This project uses an Elm-like architecture, centralizing application logic around a single `Model` and processing events through an update-view cycle, inspired by [ELM](https://guide.elm-lang.org/architecture/). 

## Asynchronous Rendering

The core of ChatTea's UI lies in its non-blocking asynchronous render loop. This design ensures that the UI remains responsive and doesn't freeze during network operations or other processes.

## See https://github.com/d-holguin/async-ratatui for a simplier, cleaner structure. 

## Elm-like Architecture

### The Message Enum

```rust
#[derive(Clone, Debug)]
pub enum Message {
    Quit,
    Error,
    Tick,
    Render,
    Key(KeyEvent),
    ReceivedNetworkMessage(String),
    SendNetworkMessage(String),
    Log(ListItem<'static>),
    RegisterUser(String),
}
```
Each user interaction and system event is represented by the Message enum, allowing for clear and structured handling of all possible events in the application.

### Model

The Model represents the state of the application, encompassing everything from UI state to network management. It acts as the central source of truth within the application, evolving over time in response to the messages processed in the update function.

```rust

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
```
### Update Function

The update function processes messages and updates the state of the Model. This function is where the logic of the application responds to user events.

```rust

pub fn update(model: &mut Model, message: Message) {
    // ... existing code ...
}
```

### View Function

The view function is responsible for rendering the UI based on the current state of the Model. This clear separation of concerns makes the code more maintainable and easier to understand.

```rust

pub fn view(frame: &mut Frame<'_>, model: &Model) {
    // ... existing code ...
}
```

