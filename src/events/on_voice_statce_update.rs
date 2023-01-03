use crate::util::Error;
use entity::DatabaseConnection;
use poise::serenity_prelude::Context;
use poise::serenity_prelude::RoleId;

pub(crate) async fn handle(
    old: &Option<poise::serenity_prelude::VoiceState>,
    new: &poise::serenity_prelude::VoiceState,
    db: &DatabaseConnection,
    ctx: &Context,
) -> Result<(), Error> {
    let mut to_remove: Vec<String> = vec![];
    let mut to_add: Vec<String> = vec![];
    if let Some(old) = old {
        let old_channel_id = old.channel_id.or(new.channel_id);
        if old_channel_id != new.channel_id {
            if let Some(old_channel_id) = old_channel_id {
                let mut r = get_roles(db, old_channel_id.0).await?;
                to_remove.append(&mut r);
            }
        }
    }
    if let Some(new_channel_id) = new.channel_id {
        let mut a = get_roles(db, new_channel_id.0).await?;
        to_add.append(&mut a);
        to_remove.retain(|v| !to_add.contains(v));
    }
    let to_remove = to_remove
        .iter()
        .map(|role| RoleId::from(role.parse::<u64>().unwrap()))
        .collect::<Vec<RoleId>>();
    let to_add = to_add
        .iter()
        .map(|role| RoleId::from(role.parse::<u64>().unwrap()))
        .collect::<Vec<RoleId>>();
    let guild = new.guild_id.expect("failed to get guild id");
    let guild = ctx.http.get_guild(guild.0).await?;
    let mut user = guild.member(ctx, new.user_id).await?;
    user.remove_roles(ctx, &to_remove).await?;
    user.add_roles(ctx, &to_add).await?;
    Ok(())
}

async fn get_roles(db: &DatabaseConnection, channel: u64) -> Result<Vec<String>, Error> {
    let vec = entity::prelude::Channel::find_by_channel(channel)
        .all(db)
        .await?;

    Ok(vec.into_iter().map(|model| model.role_id).collect())
}
