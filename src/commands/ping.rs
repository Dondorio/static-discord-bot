use crate::commands::*;
use poise::serenity_prelude as serenity;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("Ping <@{}>", u.id);
    ctx.say(response).await?;
    Ok(())
}
