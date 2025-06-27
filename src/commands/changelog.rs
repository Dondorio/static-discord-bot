use std::fmt;

use crate::commands::*;
use poise::serenity_prelude as serenity;

#[derive(poise::ChoiceParameter, Debug)]
enum Versions {
    #[name = "1.0"]
    V1x0,
}

impl fmt::Display for Versions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            Versions::V1x0 => "1.0",
        };
        write!(f, "{}", str)
    }
}

#[poise::command(slash_command, prefix_command)]
pub async fn changelog(
    ctx: Context<'_>,
    #[description = "Version"] version: Option<Versions>,
) -> Result<(), Error> {
    let response = format!("{}", version.unwrap_or(Versions::V1x0));

    println!("{:?}", ctx.data());

    {
        let mut data = ctx.data().foo.lock().await;
        *data = "not bar".to_string();
    }

    ctx.say(response).await?;
    Ok(())
}
