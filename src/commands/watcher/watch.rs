use crate::util::{edit_reply, Context, Error};
use entity::sea_orm::{ActiveModelTrait, ColumnTrait, QueryFilter};
use poise::serenity_prelude::{Channel, Role};

/// activate a channel with a certain role to add
#[poise::command(
    slash_command,
    category = "setup",
    ephemeral,
    rename = "watch",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn command(
    ctx: Context<'_>,
    #[description = "channel to watch"] channel: Channel,
    #[description = "role to assign"] role: Role,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let (updated, roles) = watch_save(ctx, channel.clone(), role).await?;

    edit_reply(ctx, |b| {
        b.embed(|e| {
            e.title(if updated {
                "successfully updated settings"
            } else {
                "nothing to change"
            })
            .description("The following roles will be added/removed for users of chosen channel:")
            .field("channel", format!("<#{}>", channel.id()), true)
            .field(
                "roles",
                roles
                    .iter()
                    .map(|role| format!("<@&{}>", role.role_id))
                    .collect::<Vec<_>>()
                    .join("\n"),
                true,
            )
        })
    })
    .await?;

    Ok(())
}

async fn watch_save(
    ctx: Context<'_>,
    channel: Channel,
    role: Role,
) -> Result<(bool, Vec<entity::channel::Model>), Error> {
    let db = ctx
        .data()
        .database
        .get()
        .expect("database connection not yet initialized");

    let guild_id = ctx.guild_id().expect("failed to get guild id").0;
    let channel = channel.id().0;
    let role = role.id.0;

    let guild = entity::channel::ActiveModel::update_role(guild_id, channel, role);
    let count = entity::channel::Entity::find_by_guild(guild_id)
        .filter(entity::channel::Column::ChannelId.eq(channel.to_string()))
        .filter(entity::channel::Column::RoleId.eq(role.to_string()))
        .all(db)
        .await?
        .len();

    if count == 0 {
        guild.save(db).await?;
    }

    let roles = entity::channel::Entity::find_by_channel(channel)
        .all(db)
        .await?;

    Ok((count == 0, roles))
}
