use models::shimmie_json::{ShimmieJson, ShimmieSections};
use handlers::comment::comment_handler;
use handlers::image::image_handler;
use serenity::all::{ChannelId, Http, Ready};
use udp_client::{UdpClient, UdpHandler};
use std::sync::Arc;

use serenity::async_trait;
use serenity::prelude::*;

use core::net::SocketAddr;

mod udp_client;
mod models;
mod handlers;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        match std::env::var("logchannelID") {
            Ok(id) =>{
                let ch = ChannelId::new(id.parse::<u64>().unwrap());
                if let Err(why) = ch.say(&ctx.http, "<:helperfox:1351307021340639374> testing from rust aaaaaaaaa").await {
                    println!("Error sending message: {why:?}");
                }
            },
            Err(_) => {
                eprintln!("No log channel id given");
            }
        };
        println!("{} is connected!", ready.user.name);
    }
}
            

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;

    let client = Arc::new(Mutex::new(
        Client::builder(&token, intents)
            .event_handler(Handler)
            .await
            .expect("Err creating client"),
    ));
    

    let handler = Arc::new(MyHandler {
        http: client.lock().await.http.clone(), 
    });

    let udpclient = UdpClient::new("0.0.0.0:10004", handler).await?;

    let discord_task = {
        let client_clone = client.clone();
        tokio::spawn(async move {
            let mut client = client_clone.lock().await; 
            if let Err(why) = client.start().await {
                println!("Client error: {why:?}");
            }
        })
    };

    let udp_task = tokio::spawn(async move {
        let _ = udpclient.run().await;
    });

    tokio::signal::ctrl_c().await?;
    discord_task.abort();
    udp_task.abort();

    Ok(())
}

pub struct MyHandler {
    http: Arc<Http>
}
#[async_trait]
impl UdpHandler for MyHandler {
    async fn on_receive(&self, _len: usize, addr: SocketAddr, msg: &[u8]) {
        match serde_json::from_slice::<ShimmieJson>(msg) {
            Ok(msg) => {
                match msg.section {
                    ShimmieSections::COMMENT => comment_handler(self.http.clone(), msg.r#type,msg.fields).await,
                    ShimmieSections::IMAGE => image_handler(self.http.clone(), msg.r#type,msg.fields).await,
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("Failed to parse JSON from {}: {}", addr, e);
            }
        }
    }
}