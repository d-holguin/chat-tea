use anyhow::Result;
pub mod tui;
pub use tui::*;

pub mod model;
pub use model::*;

pub mod view;
pub use view::*;

pub mod update;
pub use update::*;

pub mod network_manager;
pub use network_manager::*;

pub mod logging;
pub use logging::*;

pub async fn run_app(mut model: Model<'_>, mut tui: Tui) -> Result<()> {
    tui.enter()?;
    let mut should_exit = false;
    loop {
        tokio::select! {
                Some(message) = tui.next() => {
                    match message {
                        Message::Render => {
                            // Update FPS counter
                            model.fps_counter.tick();
                            // Handle the render event
                            tui.terminal.draw(|f| {
                                view(f, &model);
                            })?;
                        },
                        Message::Quit => {
                            should_exit = true;
                        },
                        message => {
                            update(&mut model, message);
                        }
                    }
                },
                Some(network_msg) = model.network_manager.get_incoming_messages().recv() => {
                    update(&mut model, Message::ReceivedNetworkMessage(network_msg));
                },
            }
        if should_exit {
            break;
        }
    }
    tui.exit()?;
    Ok(())
}
