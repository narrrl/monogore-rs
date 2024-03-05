pub mod general;

pub use general::*;

pub use crate::{Context, Result};

///
/// Ping command to see if bot is alive
///
#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    ctx.reply("pong!").await?;
    Ok(())
}
