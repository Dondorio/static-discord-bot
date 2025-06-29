use crate::{commands::*, generators};
use ::serenity::all::CreateAttachment;
use poise::CreateReply;
use tokio::{fs, time::Instant};

#[poise::command(slash_command, prefix_command)]
pub async fn static_audio(
    ctx: Context<'_>,
    #[min = 1.0]
    #[max = 60.0]
    #[description = "Length in seconds"]
    length: Option<f32>,
    #[min = 0.0]
    #[max = 2.0]
    #[description = "Amplitude (basically volume)"]
    amplitude: Option<f32>,
    #[min = 0.25]
    #[max = 1.5]
    #[description = "Sample rate multiplier (44100 * value)"]
    rate: Option<f32>,
) -> Result<(), Error> {
    let fname = format!("tmp/audio/{}.wav", ctx.author().id);

    let now = Instant::now();
    generators::audio::generate_white_noise(
        &fname,
        length.unwrap_or(1.0),
        amplitude.unwrap_or(1.0),
        rate.unwrap_or(1.0),
    )
    .await;
    let duration = now.elapsed();

    let result = CreateAttachment::path(&fname).await.unwrap();

    ctx.send(CreateReply {
        content: Some(format!("Here's your audio generated in {:?}", duration)),
        attachments: vec![result],
        ephemeral: Some(true),
        reply: true,
        ..Default::default()
    })
    .await?;

    fs::remove_file(fname).await?;

    Ok(())
}
