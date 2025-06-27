use crate::commands::*;
use crate::generators::image;
use poise::CreateReply;
use serenity::all::CreateAttachment;
use tokio::time::Instant;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn static_image(
    ctx: Context<'_>,
    #[min = 1]
    #[max = 1000]
    #[description = "The width of your image"]
    width: Option<u32>,
    #[min = 1]
    #[max = 1000]
    #[description = "The height of your image"]
    height: Option<u32>,
) -> Result<(), Error> {
    let w = width.as_ref().unwrap_or(&100);
    let h = height.as_ref().unwrap_or(&100);

    let now = Instant::now();
    image::white_noise::generate_white_noise(*w, *h)
        .await
        .unwrap();
    let duration = now.elapsed();

    let img = CreateAttachment::path("./img.png").await.unwrap();

    ctx.send(CreateReply {
        content: Some(format!("Here's your image generated in {:?}", duration)),
        attachments: vec![img],
        ephemeral: Some(true),
        reply: true,
        ..Default::default()
    })
    .await?;
    Ok(())
}
