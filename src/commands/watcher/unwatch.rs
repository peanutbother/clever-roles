use crate::util::{database, edit_reply, Context, Error};
use entity::sea_orm::ActiveModelTrait;
use poise::{
    serenity_prelude::{Channel, ChannelId, RoleId},
    AutocompleteChoice,
};

/// Disable a channel / role binding
#[poise::command(
    slash_command,
    category = "setup",
    ephemeral,
    rename = "unwatch",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn command(
    ctx: Context<'_>,
    #[description = "which assignment to delete"]
    #[autocomplete = "unwatch_autocomplete"]
    id: i32,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let (channel, role) = unwatch_save(ctx, id as u64).await?;

    edit_reply(ctx, |b| {
        b.embed(|e| {
            //
            e.title("successfully updated settings")
            .description("the following role will no longer be added/removed when entering respective channel")
            .field("channel", format!("<#{channel}>"), true)
            .field("roles", format!("<@&{role}>"), true)
        })
    })
    .await?;
    Ok(())
}

pub async fn unwatch_save(ctx: Context<'_>, id: u64) -> Result<(u64, u64), Error> {
    let db = database(ctx);
    let entry = entity::prelude::Channel::find_by_channel(id)
        .one(db)
        .await?;

    if let Some(entry) = entry {
        let model: entity::channel::ActiveModel = entry.clone().into();
        model.delete(db).await?;

        Ok((
            entry
                .clone()
                .channel_id
                .parse()
                .expect("failed to parse string into u64"),
            entry
                .role_id
                .parse()
                .expect("failed to parse string into u64"),
        ))
    } else {
        Err(Error::from(format!("No entry with id {id} found")))
    }
}

async fn unwatch_autocomplete(
    ctx: Context<'_>,
    _partial: &str,
) -> impl Iterator<Item = AutocompleteChoice<i32>> {
    let db = database(ctx);
    let guild = ctx
        .guild()
        .expect("cannot run this command outside of a guild");

    let connections = entity::channel::Entity::find_by_guild(guild.id.0)
        .all(db)
        .await
        .expect("failed to get database entries");

    connections.into_iter().map(move |m| {
        let channel = guild
            .channels
            .get(&ChannelId::from(m.channel_id.parse::<u64>().unwrap()))
            .expect("failed to retrieve channel");
        let channel = match channel {
            Channel::Guild(channel) => channel,
            _ => panic!("invalid channel"),
        };

        let role = guild
            .roles
            .get(&RoleId::from(m.role_id.parse::<u64>().unwrap()))
            .expect("failed to retrieve role");

        let name = format!("#{} -> {}", channel.name, role.name);

        AutocompleteChoice { name, value: m.id }
    })
}
