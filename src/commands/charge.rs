use crate::commands::*;

#[poise::command(slash_command, prefix_command)]
pub async fn charge(ctx: Context<'_>) -> Result<(), Error> {
    let mut lock = ctx.data().charge_count.lock().await;
    *lock += 1;

    let response = format!("The static has been charged ``{}`` times today", *lock);

    drop(lock);
    ctx.say(response).await?;
    Ok(())
}
