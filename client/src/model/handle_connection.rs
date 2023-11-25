use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

pub async fn manage_tcp_stream(
    mut stream: TcpStream,
    mut sending_message_rx: UnboundedReceiver<String>,
    incoming_msg_tx: UnboundedSender<String>,
) {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => break, // Connection was closed
                    Ok(_) => {
                        incoming_msg_tx.send(line.clone()).unwrap();
                        line.clear();
                    }
                    Err(_) => {
                        break;
                    }
                }
            },
            message = sending_message_rx.recv() => {
                if let Some(msg) = message {
                    let msg = format!("{msg}\n");
                    writer.write_all(msg.as_bytes()).await.unwrap();
                }
            },
        }
    }
}
