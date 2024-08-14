use std::collections::HashMap;
use std::sync::{Arc};

use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Server failed to run: {}", e);
        std::process::exit(1);
    }
}
#[derive(Clone)]
struct User {
    name: String,
    _id: String,
}

pub async fn run() -> Result<()> {
    let listener = TcpListener::bind("localhost:8080").await?;
    let (tx, _rx) = broadcast::channel(15);
    let user_map: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
    println!("Starting server");
    loop {
        let (mut socket, addr) = listener.accept().await?;

        let user_map_clone = user_map.clone();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let result: Result<()> = async {
                let (reader, mut writer) = socket.split();
                let mut reader = BufReader::new(reader);
                let mut line = String::new();

                let bytes_read = reader.read_line(&mut line).await?;
                if bytes_read == 0 {
                    return Ok(());
                }
                let username = line.split("username:").collect::<Vec<&str>>()[1]
                    .trim()
                    .to_string();
                let user_id = addr.to_string();
                let user = User {
                    name: username.clone(),
                    _id: user_id.clone(),
                };
                user_map_clone.lock().await.insert(user_id.clone(), user);
                println!("{} connected", username);

                let success_message = format!("Welcome to the chat, {username}!\n", );
                writer.write_all(success_message.as_bytes()).await?;
                line.clear();

                loop {
                    tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result? == 0 {
                            break;
                        }
                        if !line.trim().is_empty() {

                            let user_name = {
                                let user_map_guard = user_map_clone.lock().await;

                                if let Some(user) = user_map_guard.get(&user_id) {
                                    user.name.clone()
                                } else {
                                    eprintln!("User not found for ID: {}", user_id);
                                    continue;
                                }
                            };
                            let msg = format!("{}: {}", user_name, line.clone());
                            tx.send((msg, addr))?;
                        }
                        line.clear();
                    },
                    result = rx.recv() => {
                        let (msg, _other_addr) = result?;

                        writer.write_all(msg.as_bytes()).await?;

                    },
                }
                }
                Ok(())
            }.await;
            if let Err(e) = result {
                eprintln!("Error handling connection: {:?}", e);
            }
        });
    }
}
