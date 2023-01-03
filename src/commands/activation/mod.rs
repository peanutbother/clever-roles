use crate::util::{database, Context, Error};
use entity::sea_orm::ActiveModelTrait;

mod activate;
mod deactivate;

pub use activate::command as activate;
pub use deactivate::command as deactivate;

async fn update(ctx: Context<'_>, active: bool) -> Result<(), Error> {
    let db = database(ctx);

    let guild = entity::guild::ActiveModel::update_guild(
        ctx.guild_id().expect("failed to get guild id").0,
        active,
    );
    guild.save(db).await?;

    Ok(())
}
