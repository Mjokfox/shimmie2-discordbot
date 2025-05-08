use std::sync::Arc;

use crate::{models::shimmie_json::{Fields, ShimmieSectionTypes}, JsonHandler, create_post, delete_post, get_message_from_post_id, get_comment_messages_from_post_id, delete_comments_with_post_id, DbPool};
use serenity::all::{ChannelId, ChannelType, CreateEmbed, CreateMessage, EditMessage, Http, MessageId, Timestamp};
use mime_serde_shim::Wrapper as Mime;
use mime_guess::get_mime_extensions;

impl JsonHandler {
    pub async fn image_handler(&self,r#type: ShimmieSectionTypes, fields: Fields ) {
        match std::env::var("channelID") {
            Ok(id) =>{
                let handler = ImageHandler {
                    http: self.http.clone(),
                    db_pool: self.db_pool.clone(),
                    ch: ChannelId::new(id.parse::<u64>().unwrap())
                };
                match r#type {
                    ShimmieSectionTypes::Create => handler.create(fields).await,
                    ShimmieSectionTypes::Edit => handler.edit(fields).await,
                    ShimmieSectionTypes::Delete => handler.delete(fields).await
                }
            },
            Err(_) => {
                eprintln!("No channel id given");
            }
        };
    }
}

struct ImageHandler {
    pub http: Arc<Http>,
    pub db_pool: DbPool,
    pub ch: ChannelId
}

impl ImageHandler {
    async fn create(&self, fields: Fields) {
        let embed = self.embed(
            fields.post_id.unwrap_or_default(), 
            fields.username.unwrap_or_default(), 
            fields.hash.unwrap_or_default(), 
            fields.mime.unwrap_or(mime_serde_shim::Wrapper(mime::IMAGE_JPEG)), 
            fields.size.unwrap_or_default()
        );
        let builder = CreateMessage::new().embed(embed);
        let mess = self.ch.send_message(self.http.clone(), builder).await;
        match mess {
            Ok(message) => {
                let res = self.db_pool.get();
                match res {
                    Ok(mut conn) => {
                        create_post(&mut conn, &fields.post_id.unwrap_or_default(), &message.id.into());
                    },
                    Err(why) => println!("db ded {why:?}")
                }
                if let Ok(channel) = self.ch.to_channel(self.http.clone()).await {
                    if channel.guild().unwrap_or_default().kind == ChannelType::News {
                        let _ = message.crosspost(self.http.clone()).await;
                    }
                }
            },
            Err(why) => println!("Error sending post: {why:?}")
        }
    }

    async fn edit(&self, fields: Fields) {
        let res = self.db_pool.get();
        match res {
            Ok(mut conn) => {
                let post_id = &fields.post_id.unwrap_or_default();
                if let Ok(m) = get_message_from_post_id(&mut conn, post_id) {
                    let message_id = MessageId::new(m.try_into().unwrap());
                    let embed = self.embed(
                        fields.post_id.unwrap_or_default(), 
                        fields.username.unwrap_or_default(), 
                        fields.hash.unwrap_or_default(), 
                        fields.mime.unwrap_or(mime_serde_shim::Wrapper(mime::IMAGE_JPEG)), 
                        fields.size.unwrap_or_default()
                    );
                    let builder = EditMessage::new().embed(embed);
                    let _ = self.ch.edit_message(self.http.clone(), message_id, builder).await;
                }
            },
            Err(_) => println!("Error editing post")
        }
    }

    async fn delete(&self, fields: Fields) {
        let res = self.db_pool.get();
        match res {
            Ok(mut conn) => {
                let post_id = &fields.post_id.unwrap_or_default();
                if let Ok(m) = get_message_from_post_id(&mut conn, post_id) {
                    let message_id = MessageId::new(m.try_into().unwrap());
                    let _ = self.ch.delete_message(self.http.clone(), message_id).await;
                    delete_post(&mut conn, post_id);
                }
                if let Ok(mvec) = get_comment_messages_from_post_id(&mut conn, post_id) {
                    for &m in mvec.iter(){
                        let message_id = MessageId::new(m.try_into().unwrap());
                        let _ = self.ch.delete_message(self.http.clone(), message_id).await;
                    }
                }
                delete_comments_with_post_id(&mut conn, post_id);
            },
            Err(_) => println!("Post deletion failed")
        }
    }

    fn get_supported_extension(mime: &Mime) -> Option<&'static str> {
        let supported_extensions = ["jpg", "jpeg", "png", "gif", "webp", "mp4", "webm", "mov"]; // Add only the extensions you want
    
        get_mime_extensions(mime)
            .and_then(|exts| exts.iter().find(|&&ext| supported_extensions.contains(&ext)).copied())
    }
    
    fn embed(&self, post_id: i32, username: String, hash: String, post_mime: Mime, size: i32) -> CreateEmbed {
        let server_url = match std::env::var("serverUrl") {
            Ok(a) => a,
            Err(_) => {"https://example.com".to_string()}
        };
        let mut path = "thumbs";
        let mut fext = "jpg";
        if post_mime.type_() != mime::VIDEO {
            fext = Self::get_supported_extension(&post_mime).unwrap_or("jpg");
            if size < 10485760 || fext == "gif" {
                path = "images";
            }
        }
        CreateEmbed::new()
            .color(0xff8c00)
            .title(format!("New post! >>{}", post_id))
            .url(format!("{}/post/view/{}",server_url,post_id))
            .image(format!("{}/{}/{}/{}.{}",server_url,path,hash,post_id,fext))
            .description(format!("By [{}]({}/user/{})",username,server_url,username))
            .timestamp(Timestamp::now())
    }
}
