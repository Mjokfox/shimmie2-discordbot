use std::sync::Arc;

use crate::{models::shimmie_json::{Fields, ShimmieSectionTypes}, JsonHandler, create_comment, delete_comment, get_message_from_comment_id, get_message_from_post_id, DbPool};
use serenity::all::{ChannelId, CreateEmbed, CreateMessage, EditMessage, Http, MessageId, MessageReference, MessageReferenceKind, Timestamp};

impl JsonHandler {
    pub async fn comment_handler(&self,r#type: ShimmieSectionTypes, fields: Fields ) {
        match std::env::var("channelID") {
            Ok(id) =>{
                let handler = CommentHandler {
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


struct CommentHandler {
    pub http: Arc<Http>,
    pub db_pool: DbPool,
    pub ch: ChannelId
}

impl CommentHandler {
    async fn create(&self, fields: Fields) {
        let embed = self.embed(
            fields.post_id.unwrap_or_default(), 
            fields.username.unwrap_or_default(), 
            fields.comment_id.unwrap_or_default(), 
            fields.message.unwrap_or_default()
        );
        let mut builder = CreateMessage::new().embed(embed);

        let res = self.db_pool.get();
        match res {
            Ok(mut conn) => {
                let post_id = &fields.post_id.unwrap_or_default();
                if let Ok(m) = get_message_from_post_id(&mut conn, post_id){
                    let reference = MessageReference::new(MessageReferenceKind::default(), self.ch).message_id(MessageId::new(m.try_into().unwrap()));
                    builder = builder.reference_message(reference);
                }
                let mess = self.ch.send_message(self.http.clone(), builder).await;
                match mess {
                    Ok(message) => {
                    create_comment(&mut conn, &fields.comment_id.unwrap_or_default(), &fields.post_id.unwrap_or_default(), &message.id.into());
                    },
                    Err(why) => println!("Error sending comment: {why:?}")
                }
            },
            Err(why) => println!("db ded {why:?}")
        }
        
    }

    async fn edit(&self, fields: Fields) {
        let res = self.db_pool.get();
        match res {
            Ok(mut conn) => {
                let comment_id = &fields.comment_id.unwrap_or_default();
                if let Ok(m) = get_message_from_comment_id(&mut conn, comment_id) {
                    let message_id = MessageId::new(m.try_into().unwrap());
                    let embed = self.embed(
                        fields.post_id.unwrap_or_default(),
                        fields.username.unwrap_or_default(),
                        fields.comment_id.unwrap_or_default(),
                        fields.message.unwrap_or_default()
                    );
                    let builder = EditMessage::new().embed(embed);
                    let _ = self.ch.edit_message(self.http.clone(), message_id, builder).await;
                }
            },
            Err(_) => println!("Comment editing failed")
        }
    }

    async fn delete(&self, fields: Fields) {
        let res = self.db_pool.get();
        match res {
            Ok(mut conn) => {
                let comment_id = &fields.comment_id.unwrap_or_default();
                if let Ok(m) = get_message_from_comment_id(&mut conn, comment_id) {
                    let message_id = MessageId::new(m.try_into().unwrap());
                    let _ = self.ch.delete_message(self.http.clone(), message_id).await;
                    delete_comment(&mut conn, comment_id);
                }
            },
            Err(_) => println!("Comment deleting failed")
        }
    }

    fn embed(&self, post_id: i32, username: String, comment_id: i32, comment: String) -> CreateEmbed {
        let server_url = match std::env::var("serverUrl") {
            Ok(a) => a,
            Err(_) => {"https://example.com".to_string()}
        };
        CreateEmbed::new()
            .color(0xff8c00)
            .title(format!("New comment on post >>{}", post_id))
            .url(format!("{}/post/view/{}#{}",server_url,post_id,comment_id))
            .fields(vec![
                (username, comment, true),
            ])
            .timestamp(Timestamp::now())
    }
}