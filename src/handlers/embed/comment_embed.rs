use serenity::all::{CreateEmbed, CreateMessage, Timestamp};


pub fn comment_embed(post_id: i64, username: String, comment_id: i64, comment: String) -> CreateMessage {
    let server_url = match std::env::var("serverUrl") {
        Ok(a) => a,
        Err(_) => {"https://example.com".to_string()}
    };
    let embed = CreateEmbed::new()
        .color(0xff8c00)
        .title(format!("New comment on post >>{}", post_id))
        .url(format!("{}/post/view/{}#{}",server_url,post_id,comment_id))
        .fields(vec![
            (username, comment, true),
        ])
        .timestamp(Timestamp::now());
    CreateMessage::new()
        .embed(embed)
}