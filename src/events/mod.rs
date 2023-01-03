use crate::util::{Data, Error};
use poise::{serenity_prelude::Context, Event};

mod on_channel_delete;
mod on_guild_delete;
mod on_ready;
mod on_voice_statce_update;

pub async fn handle<'a>(ctx: &Context, event: &Event<'a>, data: &Data) -> Result<(), Error> {
    let db = data
        .database
        .get()
        .expect("failed to retrieve database connection");

    match event {
        Event::VoiceStateUpdate { new, old } => {
            on_voice_statce_update::handle(old, new, db, ctx).await
        }
        Event::Ready { .. } => on_ready::handle().await,
        Event::GuildDelete { incomplete, .. } => on_guild_delete::handle(incomplete, db).await,
        Event::ChannelDelete { channel } => on_channel_delete::handle(channel, db).await,
        _ => Ok(()),
    }
}
