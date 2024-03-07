use crate::commands::*;

///
/// Searches for a riven and its average selling price
///
#[poise::command(prefix_command, slash_command, track_edits, category = "warframe")]
pub async fn riven(
    ctx: Context<'_>,
    #[description = "the riven you are searching for"] riven_name: String,
) -> Result<()> {
    let warframe_api = ctx.data().warframe_api.lock().await;
    Ok(())
}
