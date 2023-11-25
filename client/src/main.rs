use client::{Model, NetworkManager, Tui};
use color_eyre::Result;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub async fn run() -> Result<()> {
    let tui = Tui::new(4.0, 30.0)?;
    let network_manager = NetworkManager::connect_to_server("localhost:8080").await?;
    let app = Model::new(&tui, network_manager);

    app.start(tui).await?;
    Ok(())
}
