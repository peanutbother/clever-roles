use crate::constants;
use crate::util::Context;
use crate::util::Error;
use poise::samples::HelpConfiguration;

/// Display bot help
#[poise::command(slash_command, category = "info", ephemeral, rename = "help")]
pub async fn command(
    ctx: Context<'_>,
    #[description = "get help for a specific command"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    match command {
        Some(command) => {
            poise::builtins::help(
                ctx,
                Some(command.as_str()),
                HelpConfiguration {
                    extra_text_at_bottom: format!("{} v{}", constants::NAME, constants::VERSION)
                        .as_str(),
                    ..Default::default()
                },
            )
            .await
        }

        None => {
            poise::builtins::help(
                ctx,
                None,
                HelpConfiguration {
                    extra_text_at_bottom: format!("{} v{}", constants::NAME, constants::VERSION)
                        .as_str(),
                    ..Default::default()
                },
            )
            .await
        }
    }?;

    Ok(())
}
