use colored::Colorize;
use poise::serenity_prelude::*;
use serenity::all::GuildId;
use serenity::{async_trait, model::gateway::Ready};
use std::env;
use tokio::{fs, sync::Mutex};

mod commands;
mod generators;

struct Handler;

#[derive(Debug)]
pub struct Data {
    charge_count: Mutex<u32>,
}

#[async_trait]
impl EventHandler for Handler {
    // async fn message(&self, ctx: Context, msg: Message) {}

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name.bold().purple());
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Ensure that tmp/audio and tmp/image exist for both generators
    fs::create_dir_all("tmp/audio")
        .await
        .expect("Failed to create tmp/audio");
    fs::create_dir_all("tmp/image")
        .await
        .expect("Failed to create tmp/image");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::changelog::changelog(),
                commands::charge::charge(),
                commands::ping::ping(),
                commands::sermon::sermon(),
                commands::static_audio::static_audio(),
                commands::static_image::static_image(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    GuildId::new(
                        env::var("GUILD_ID")
                            .expect("Expected GUILD_ID in environment")
                            .parse()
                            .expect("GUILD_ID must be an integer"),
                    ),
                )
                .await?;
                Ok(Data {
                    charge_count: Mutex::new(0),
                })
            })
        })
        .build();

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
