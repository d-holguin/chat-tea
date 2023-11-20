#[tokio::main]
async fn main() {
    if let Err(e) = client::app::run().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
