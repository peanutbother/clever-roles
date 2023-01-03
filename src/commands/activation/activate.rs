use crate::util::edit_reply;

use super::update;

use crate::util::Error;

use crate::util::Context;

/// activate bot for this server
#[poise::command(
    slash_command,
    category = "setup",
    ephemeral,
    rename = "activate",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn command(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    update(ctx, true).await?;
    edit_reply(ctx, |b| b.content("activated bot for this guild!")).await?;

    Ok(())
}
