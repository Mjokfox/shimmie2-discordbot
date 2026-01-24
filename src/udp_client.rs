use std::sync::Arc;
use serenity::async_trait;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::task;

#[async_trait]
pub trait UdpHandler: Send + Sync {
    async fn on_receive(&self, msg: &[u8]);
}

pub struct UdpClient<H: UdpHandler + 'static> {
    socket: Arc<UdpSocket>,
    handler: Arc<H>,
    threads: Vec<tokio::task::JoinHandle<()>>
}

impl<H: UdpHandler + 'static> UdpClient<H> {
    pub async fn new(addr: &str, handler: Arc<H>) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        println!("UDP Client bound to {}", addr);
        Ok(Self {
            socket: Arc::new(socket),
            handler,
            threads: Vec::<tokio::task::JoinHandle<()>>::default()
        })
    }

    pub fn run(&mut self) {
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(20);
        let socket = Arc::clone(&self.socket);
        let handler = Arc::clone(&self.handler);
        self.threads.push(task::spawn(async move {
            let mut buf = [0; 65535];
            
            loop {
                if let Ok((len, _)) = socket.recv_from(&mut buf).await  {
                    if let Err(e) = tx.send(buf[..len].to_vec()).await {
                        eprintln!("Failed to push received data: {}", e);
                    }

                }
            }
        }));

        self.threads.push(task::spawn(async move {
            loop {
                if let Some(msg) = rx.recv().await {
                    handler.on_receive(&msg).await;
                }
            }
        }));
    }

    pub fn stop(self) {
        for t in self.threads {
            t.abort();
        }
    }
}
