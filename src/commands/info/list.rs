use std::collections::HashMap;

use crate::util::{database, edit_reply, Context, Error};
use poise::serenity_prelude::CreateEmbed;

/// List every active channel with their respective roles
#[poise::command(slash_command, category = "info", ephemeral, rename = "list")]
pub async fn command(ctx: Context<'_>) -> Result<(), Error> {
    let db = database(ctx);
    ctx.defer_ephemeral().await?;

    let handles = entity::prelude::Channel::find_by_guild(
        ctx.guild_id().expect("failed to retrieve guild id").0,
    )
    .all(db)
    .await?;

    let mut watch_handles: HashMap<String, Vec<String>> = HashMap::new();
    handles.iter().for_each(|m| {
        if let Some(roles) = watch_handles.get(&m.channel_id) {
            let mut roles = roles.clone();
            let new_roles = vec![m.role_id.clone()];
            roles.extend(new_roles.into_iter());

            watch_handles.insert(m.channel_id.clone(), roles.to_owned());
        } else {
            watch_handles.insert(m.channel_id.clone(), vec![m.role_id.clone()]);
        }
    });

    edit_reply(ctx, |b| {
        //
        let watch_handles: Vec<CreateEmbed> = watch_handles
            .keys()
            .zip(watch_handles.values())
            .map(|(channel_id, role_id)| {
                CreateEmbed::default()
                    .title("active channels with roles")
                    .description("the following roles will be given, when entered the channel:")
                    .field("channel", format!("<#{}>", channel_id), true)
                    .field(
                        "role",
                        role_id
                            .iter()
                            .map(|role| format!("<@&{}>", role))
                            .collect::<Vec<_>>()
                            .join("\n"),
                        true,
                    )
                    .to_owned()
            })
            .collect();

        b.add_embeds(if !watch_handles.is_empty() {
            watch_handles
        } else {
            vec![
                // fallback embed
                CreateEmbed::default()
                    .title("active channels with roles")
                    .description("currently no active channel configs are present")
                    .to_owned(),
            ]
        })
    })
    .await?;

    Ok(())
}
