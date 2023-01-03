use crate::util::{Command, Context};
use entity::sea_orm::{ActiveModelTrait, Set};

pub mod activation;
pub mod info;
pub mod watcher;

pub async fn pre_command(ctx: Context<'_>) {
    let db = crate::util::database(ctx);
    let guild_id = ctx.guild_id().expect("failed to get guild id").0;
    let guild = entity::guild::ActiveModel {
        id: Set(guild_id.to_string()),
        ..Default::default()
    };
    guild.insert(db).await.ok();
}

pub fn prepare() -> Vec<Command> {
    vec![
        activation::activate(),
        activation::deactivate(),
        info::list(),
        info::help(),
        watcher::watch(),
        watcher::unwatch(),
    ]
}
