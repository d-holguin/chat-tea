use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    run("127.0.0.1:8080").await
}

async fn run(addr: &str) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = io::split(stream);

    let mut lines_from_server = io::BufReader::new(reader).lines();
    let mut lines_from_stdin = io::BufReader::new(io::stdin()).lines();

    loop {
        select! {
            line = lines_from_server.next_line() => match line {
                Ok(Some(line)) => println!("{}", line),
                Ok(None) => break,
                Err(e) => eprintln!("Error reading from server: {}", e),
            },
            line = lines_from_stdin.next_line() => match line {
                Ok(Some(line)) => {
                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
                Ok(None) | Err(_) => break,
            },
        }
    }

    Ok(())
}
