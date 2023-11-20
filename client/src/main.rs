#[tokio::main]
async fn main() {
    if let Err(e) = client::tui::run().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
