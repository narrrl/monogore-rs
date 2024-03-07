pub mod general;
pub mod warframe;

pub use general::*;
pub use warframe::*;

pub use crate::{Context, Result};
pub use poise::serenity_prelude as serenity;

///
/// Ping command to see if bot is alive
///
#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    ctx.reply("pong!").await?;
    Ok(())
}
