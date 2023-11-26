use client::{Model, NetworkManager, Tui, TuiLogLayer};
use color_eyre::Result;
use tracing_subscriber::{fmt, layer::SubscriberExt, Registry};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub async fn run() -> Result<()> {
    let tui = Tui::new(4.0, 30.0)?;


    let log_layer = TuiLogLayer {
        message_tx: tui.event_tx.clone(),
    };

    let subscriber = Registry::default().with(log_layer);
    tracing::subscriber::set_global_default(subscriber)?;

    let network_manager = NetworkManager::connect_to_server("localhost:8080").await?;
    let app = Model::new(&tui, network_manager);
    app.start(tui).await?;

    Ok(())
}
