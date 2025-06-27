use std::env;

use colored::Colorize;
use serenity::all::{GuildId, MessageBuilder};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
// use serenity::prelude::*;
use poise::serenity_prelude::*;
use tokio::sync::Mutex;

mod commands;
mod generators;

struct Handler;

#[derive(Debug)]
pub struct Data {
    foo: Mutex<String>,
} // User data, which is stored and accessible in all command invocations
//

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut charge_count = 0;

        if msg.content.trim().to_lowercase() == "$charge" {
            charge_count += 1;
            let response = MessageBuilder::new()
                .push("The static has been charged ")
                .push_bold_safe(charge_count.to_string())
                .push(" times today")
                .build();

            if let Err(err) = msg.channel_id.say(&ctx.http, &response).await {
                println!("Error sending message: {:?}", err);
            }
        }
    }

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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::changelog::changelog(),
                commands::ping::ping(),
                commands::sermon::sermon(),
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
                    foo: Mutex::new(String::from("Bar")),
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
