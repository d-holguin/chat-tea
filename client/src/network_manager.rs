use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use tracing::{error, info};

pub struct NetworkManager {
    _incoming_msg_tx: UnboundedSender<String>,
    incoming_msg_rx: UnboundedReceiver<String>,
    sending_msg_tx: UnboundedSender<String>,
}

impl NetworkManager {
    pub async fn connect_to_server(addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;

        let (incoming_msg_tx, incoming_msg_rx) = mpsc::unbounded_channel();
        let (sending_msg_tx, sending_msg_rx) = mpsc::unbounded_channel();

        let incoming_msg_tx_clone = incoming_msg_tx.clone();
        tokio::spawn(async move {
            if let Err(e) =
                Self::read_and_write_stream(stream, incoming_msg_tx_clone, sending_msg_rx).await
            {
                error!("Failed to read and write stream: {}", e);
            }
        });

        Ok(Self {
            _incoming_msg_tx: incoming_msg_tx.clone(),
            incoming_msg_rx,
            sending_msg_tx,
        })
    }

    async fn read_and_write_stream(
        mut stream: TcpStream,
        incoming_msg_tx: UnboundedSender<String>,
        mut sending_msg_rx: UnboundedReceiver<String>,
    ) -> Result<()> {
        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            tokio::select! {
                result = reader.read_line(&mut line) => {
                    match result {
                        Ok(0) => {}
                        Ok(_) => {
                            info!("Received message: {}", line);
                            if let Err(e) = incoming_msg_tx.send(line.clone()){
                                error!("Failed to send incoming message: {}", e);
                            }
                            line.clear();
                        }
                        Err(_) => {}
                    }
                },
                message = sending_msg_rx.recv() => {
                    if let Some(msg) = message {
                        info!("Sending message: {}", msg);
                        let msg = format!("{msg}\n");
                        if let Err(e) = writer.write_all(msg.as_bytes()).await {
                            error!("Failed to send message: {}", e);
                        }
                    }
                },
            }
        }
    }
    pub fn send_message(&self, message: String) {
        let sender = self.sending_msg_tx.clone();
        tokio::spawn(async move {
            let _ = sender.send(message);
        });
    }

    pub fn get_incoming_messages(&mut self) -> &mut UnboundedReceiver<String> {
        &mut self.incoming_msg_rx
    }
}
