use client::{run_app, Model, NetworkManager, Tui, TuiLogLayer};
use anyhow::{Context, Result};
use tracing_subscriber::{layer::SubscriberExt, Registry};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub async fn run() -> Result<()> {
    let tui = Tui::new(4.0, 30.0).context("Failed to initialize the terminal user interface (TUI)")?;

    let log_layer = TuiLogLayer {
        message_tx: tui.event_tx.clone(),
    };

    let subscriber = Registry::default().with(log_layer);
    tracing::subscriber::set_global_default(subscriber)?;

    let network_manager = NetworkManager::connect_to_server("localhost:8080")
        .await
        .context("Failed to connect to the network server at localhost:8080")?;

    let model = Model::new(&tui, network_manager);

    run_app(
        model, tui,
    ).await.context("Failed to start the application")?;


    Ok(())
}
