use crate::commands::*;
use crate::generators::*;
use poise::CreateReply;
use serenity::all::CreateAttachment;
use tokio::time::Instant;

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

    let fname = format!("tmp/{}.png", ctx.author().id);

    let now = Instant::now();
    image::generate(*w, *h, &fname, Generators::White)
        .await
        .unwrap();
    let duration = now.elapsed();

    let result = CreateAttachment::path(&fname).await.unwrap();

    ctx.send(CreateReply {
        content: Some(format!("Here's your image generated in {:?}", duration)),
        attachments: vec![result],
        ephemeral: Some(true),
        reply: true,
        ..Default::default()
    })
    .await?;
    Ok(())
}
