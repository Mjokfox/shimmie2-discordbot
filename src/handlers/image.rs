use std::sync::Arc;

use crate::{
    create_post, delete_comments_with_post_id, delete_post, get_comment_messages_from_post_id,
    get_message_from_post_id,
    models::shimmie_json::{Fields, HandlerTrait},
    DbPool,
};
use mime_guess::get_mime_extensions;
use mime_serde_shim::Wrapper as Mime;
use serde::de::Error;
use serenity::all::{
    ChannelId, ChannelType, CreateEmbed, CreateMessage, EditMessage, Http, MessageId, Timestamp,
};
use crate::errors::MjokError;

pub struct ImageHandler {
    pub http: Arc<Http>,
    pub db_pool: DbPool,
    pub ch: ChannelId,
    pub server_url: String,
}

impl HandlerTrait for ImageHandler {
    async fn create(&self, fields: Fields) {
        if let Err(why) = self.try_create(fields).await {
            println!("db ded {why:?}");
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
                        fields
                            .mime
                            .unwrap_or(mime_serde_shim::Wrapper(mime::IMAGE_JPEG)),
                        fields.size.unwrap_or_default(),
                    );
                    let builder = EditMessage::new().embed(embed);
                    let _ = self
                        .ch
                        .edit_message(self.http.clone(), message_id, builder)
                        .await;
                }
            }
            Err(_) => println!("Error editing post"),
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
                    for &m in mvec.iter() {
                        let message_id = MessageId::new(m.try_into().unwrap());
                        let _ = self.ch.delete_message(self.http.clone(), message_id).await;
                    }
                }
                delete_comments_with_post_id(&mut conn, post_id);
            }
            Err(_) => println!("Post deletion failed"),
        }
    }
}


impl ImageHandler {
    async fn try_create(&self, fields: Fields) -> Result<(), Box<dyn std::error::Error>> {
        let embed = self.embed_fields(&fields);
        let mut conn = self.db_pool.get()?;

        let post_id = &fields.post_id.unwrap_or_default();
        let existing = get_message_from_post_id(&mut conn, post_id);
        if let Ok(m) = existing {
            let message_id = MessageId::new(m.try_into()?);
            let builder = EditMessage::new().embed(embed);
            self.ch
                .edit_message(self.http.clone(), message_id, builder)
                .await?;
        } else {
            let builder = CreateMessage::new().embed(embed);
            let message = self.ch.send_message(&self.http, builder).await?;
            let channel = self.ch.to_channel(&self.http).await?;
            let guild_kind = channel.guild().ok_or(MjokError::new_str("Guild Missing"))?.kind;
            if  guild_kind == ChannelType::News {
                message.crosspost(&self.http).await?;
            }
            create_post(&mut conn, post_id, &message.id.into());
        }
        Ok(())
    }

    fn get_supported_extension(mime: &Mime) -> Option<&'static str> {
        let supported_extensions = ["jpg", "jpeg", "png", "gif", "webp", "mp4", "webm", "mov"]; // Add only the extensions you want

        get_mime_extensions(mime).and_then(|exts| {
            exts.iter()
                .find(|&&ext| supported_extensions.contains(&ext))
                .copied()
        })
    }

    fn embed_fields(&self, fields: &Fields) -> CreateEmbed {
        let fields: Fields = fields.clone();
        self.embed(
            fields.post_id.unwrap_or_default(),
            fields.username.unwrap_or_default(),
            fields.hash.unwrap_or_default(),
            fields
                .mime
                .unwrap_or(mime_serde_shim::Wrapper(mime::IMAGE_JPEG)),
            fields.size.unwrap_or_default(),
        )
    }

    fn embed(
        &self,
        post_id: i32,
        username: String,
        hash: String,
        post_mime: Mime,
        size: i32,
    ) -> CreateEmbed {
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
            .url(format!("{}/post/view/{}", self.server_url, post_id))
            .image(format!(
                "{}/{}/{}/{}.{}",
                self.server_url, path, hash, post_id, fext
            ))
            .description(format!(
                "By [{}]({}/user/{})",
                username, self.server_url, username
            ))
            .timestamp(Timestamp::now())
    }
}
