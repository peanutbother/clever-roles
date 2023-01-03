use entity::DatabaseConnection;
use poise::serenity_prelude::{
    ApplicationCommandInteraction, CreateInteractionResponseFollowup, Message,
};
use std::str::FromStr;
use tokio::sync::OnceCell;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;

pub fn env_var<T: FromStr>(name: &str) -> Result<T, Error>
where
    T::Err: std::fmt::Display,
{
    Ok(std::env::var(name)
        .map_err(|_| format!("Missing {}", name))?
        .parse()
        .map_err(|e| format!("Invalid {}: {}", name, e))?)
}

#[derive(Debug)]
pub struct Data {
    pub database: OnceCell<DatabaseConnection>,
}

pub fn get_interaction(ctx: Context<'_>) -> Option<&'_ ApplicationCommandInteraction> {
    let application_context = match ctx {
        poise::Context::Application(ctx) => Some(ctx),
        poise::Context::Prefix(_) => None,
    };
    if let Some(ctx) = application_context {
        let interaction = match ctx.interaction {
            poise::ApplicationCommandOrAutocompleteInteraction::ApplicationCommand(interaction) => {
                Some(interaction)
            }
            _ => None,
        };

        return interaction;
    }
    None
}

pub async fn edit_reply<'a, F>(ctx: Context<'_>, b: F) -> Result<Message, Error>
where
    for<'b> F: FnOnce(
        &'b mut CreateInteractionResponseFollowup<'a>,
    ) -> &'b mut CreateInteractionResponseFollowup<'a>,
{
    if let Some(interaction) = get_interaction(ctx) {
        return interaction
            .create_followup_message(ctx, b)
            .await
            .map_err(|err| err.into());
    };
    Err(Error::from("failed to send reply"))
}

pub fn database(ctx: Context<'_>) -> &'_ DatabaseConnection {
    ctx.data()
        .database
        .get()
        .expect("failed to retrieve database connection")
}
