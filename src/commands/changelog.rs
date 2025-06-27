use std::{collections::HashMap, fs};

use crate::commands::*;

#[derive(Debug, poise::ChoiceParameter)]
enum Versions {
    #[name = "1.0.0.0"]
    V1x0x0x0,
    #[name = "1.0.1.0"]
    V1x0x1x0,
    #[name = "1.0.1.0A"]
    V1x0x1x0A,
    #[name = "1.1.0.0"]
    V1x1x0x0,
}

#[poise::command(slash_command, prefix_command)]
pub async fn changelog(
    ctx: Context<'_>,
    #[description = "Version"] version: Option<Versions>,
) -> Result<(), Error> {
    let file = fs::read_to_string("resources/changelog.toml")?;
    let map: HashMap<String, String> = toml::from_str(file.as_str())?;

    ctx.say(
        map.get(&format!("{:?}", version.unwrap_or(Versions::V1x1x0x0)))
            .unwrap_or(&"Failed to get changelog".to_string()),
    )
    .await?;
    Ok(())
}
