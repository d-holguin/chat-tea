#[tokio::main]
async fn main() {
    let mut app = client::App::new().expect("Failed to create app");
    if let Err(e) = app.run().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
