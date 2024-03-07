use commands::*;
use core::warframe::WarframeAPI;
use poise::serenity_prelude as serenity;

use std::sync::Arc;
use tokio::sync::Mutex;

///
/// Command modules
///
mod commands;

///
/// Core utility for the discord bot
///
mod core;

///
/// Stores data that can be accessed by any command
///
pub struct Data {
    pub warframe_api: Arc<Mutex<WarframeAPI>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> std::result::Result<(), serenity::Error> {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::privileged() | serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), uwu()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    warframe_api: Arc::new(Mutex::new(WarframeAPI::new())),
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;
    client.start().await
}
