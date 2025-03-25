use serenity::all::{CreateEmbed, CreateMessage, Timestamp};
use mime_serde_shim::Wrapper as Mime;
use mime_guess::get_mime_extensions;

fn get_supported_extension(mime: &Mime) -> Option<&'static str> {
    let supported_extensions = ["jpg", "jpeg", "png", "gif", "webp", "mp4", "webm", "mov"]; // Add only the extensions you want

    get_mime_extensions(mime)
        .and_then(|exts| exts.iter().find(|&&ext| supported_extensions.contains(&ext)).copied())
}

pub fn image_embed(post_id: i64, username: String, hash: String, post_mime: Mime, size: i64) -> CreateMessage {
    let server_url = match std::env::var("serverUrl") {
        Ok(a) => a,
        Err(_) => {"https://example.com".to_string()}
    };
    let mut path = "thumbs";
    let mut fext = "jpg";
    if post_mime.type_() != mime::VIDEO {
        fext = get_supported_extension(&post_mime).unwrap_or("jpg");
        if size < 10485760 || fext == "gif" {
            path = "images";
        }
    }
    let embed = CreateEmbed::new()
        .color(0xff8c00)
        .title(format!("New post! >>{}", post_id))
        .url(format!("{}/post/view/{}",server_url,post_id))
        .image(format!("{}/{}/{}/{}.{}",server_url,path,hash,post_id,fext))
        .description(format!("By [{}]({}/user/{})",username,server_url,username))
        .timestamp(Timestamp::now());
    CreateMessage::new()
        .embed(embed)
}