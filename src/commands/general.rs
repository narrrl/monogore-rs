use crate::commands::*;

use uwuifier::uwuify_str_sse;

///
/// UwUifies given text
///
#[poise::command(slash_command, prefix_command, track_edits)]
pub async fn uwu(
    ctx: Context<'_>,
    #[description = "Text that gets uwuified"] content: String,
) -> Result<()> {
    let content_uwuified = uwuify_str_sse(&content);
    ctx.reply(content_uwuified).await?;
    Ok(())
}
