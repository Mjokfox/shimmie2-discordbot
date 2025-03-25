use std::sync::Arc;
use serenity::async_trait;
use tokio::net::UdpSocket;
use tokio::task;
use core::net::SocketAddr;

#[async_trait]
pub trait UdpHandler: Send + Sync {
    async fn on_receive(&self, len: usize, addr: SocketAddr, msg: &[u8]);
}

pub struct UdpClient<H: UdpHandler + 'static> {
    socket: Arc<UdpSocket>,
    handler: Arc<H>
}

impl<H: UdpHandler + 'static> UdpClient<H> {
    pub async fn new(addr: &str, handler: Arc<H>) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        println!("UDP Client bound to {}", addr);
        Ok(Self {
            socket: Arc::new(socket),
            handler,
        })
    }

    pub fn run(self) -> tokio::task::JoinHandle<()> {
        let socket_clone = Arc::clone(&self.socket);
        let handler_clone = Arc::clone(&self.handler);

        task::spawn(async move {
            let mut buf = [0; 65535];

            loop {
                match socket_clone.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
                        let received_data = buf[..len].to_vec();
                        let handler_clone = Arc::clone(&handler_clone);

                        tokio::spawn(async move {
                            handler_clone.on_receive(len, addr, &received_data).await;
                        });
                    }
                    Err(e) => eprintln!("Error receiving data: {}", e),
                }
            }
        })
    }
}
