use dbfn::*;
use handlers::{comment::CommentHandler, image::ImageHandler, logging::LoggingHandler, user::UserHandler};
use models::shimmie_json::{HandlerTrait, ShimmieSectionTypes};
use models::shimmie_json::{ShimmieJson, ShimmieSections, HandlerEnum};
use serenity::all::{ChannelId, Http, Ready};
use udp_client::{UdpClient, UdpHandler};
use std::sync::Arc;
use dotenvy::dotenv;
use std::env;

use serenity::async_trait;
use serenity::prelude::*;

use core::net::SocketAddr;

mod dbfn;
pub mod schema;
mod udp_client;
mod models;
mod handlers;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        match env::var("logchannelID") {
            Ok(id) =>{
                let ch = ChannelId::new(id.parse::<u64>().unwrap());
                if let Err(why) = ch.say(&ctx.http, "<:helperfox:1351307021340639374> hewo from rust uwu").await {
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
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let ch = ChannelId::new(
        std::env::var("channelID")
        .expect("Expected a channelID in the environment")
        .parse::<u64>()
        .unwrap()
    );
    let lch = ChannelId::new(
        std::env::var("logchannelID")
        .expect("Expected a logchannelID in the environment")
        .parse::<u64>()
        .unwrap()
    );
    let server_url = std::env::var("serverUrl").expect("Expected a serverUrl in the environment");
    let udp_url = std::env::var("updUrl").expect("Expected a updUrl in the environment");

    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;

    let client = Arc::new(Mutex::new(
        Client::builder(&token, intents)
            .event_handler(Handler)
            .await
            .expect("Err creating client"),
    ));
    
    let connection = establish_pool();

    let handler = Arc::new(JsonHandler {
        http: client.lock().await.http.clone(), 
        db_pool: connection,
        ch,
        lch,
        server_url
    });

    let udpclient = UdpClient::new(&udp_url, handler).await?;

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

struct JsonHandler {
    http: Arc<Http>,
    db_pool: DbPool,
    ch: ChannelId,
    lch: ChannelId,
    server_url: String
}
#[async_trait]
impl UdpHandler for JsonHandler {
    async fn on_receive(&self, _len: usize, addr: SocketAddr, msg: &[u8]) {
        match serde_json::from_slice::<ShimmieJson>(msg) {
            Ok(msg) => {
                let handler = match msg.section {
                    ShimmieSections::Comment => HandlerEnum::Comment(CommentHandler { http: self.http.clone(), db_pool: self.db_pool.clone(), ch: self.ch, server_url: self.server_url.clone() }),
                    ShimmieSections::Post => HandlerEnum::Post(ImageHandler { http: self.http.clone(), db_pool: self.db_pool.clone(), ch: self.ch, server_url: self.server_url.clone() }),
                    ShimmieSections::User => HandlerEnum::User(UserHandler { http: self.http.clone(), ch: self.lch, server_url: self.server_url.clone() }),
                    ShimmieSections::Log => HandlerEnum::Log(LoggingHandler { }),
                };
                match msg.r#type {
                    ShimmieSectionTypes::Create => handler.create(msg.fields).await,
                    ShimmieSectionTypes::Edit => handler.edit(msg.fields).await,
                    ShimmieSectionTypes::Delete => handler.delete(msg.fields).await
                }
            }
            Err(e) => {
                eprintln!("Failed to parse JSON from {}: {}", addr, e);
            }
        }
    }
}