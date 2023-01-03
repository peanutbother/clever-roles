use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Select, Set};

pub use crate::generated::channel::*;
use crate::generated::prelude::Channel;

impl ActiveModel {
    pub fn update_role(guild: u64, channel: u64, role: u64) -> Self {
        Self {
            guild_id: Set(guild.to_string()),
            channel_id: Set(channel.to_string()),
            role_id: Set(role.to_string()),
            ..Default::default()
        }
    }
}

impl Channel {
    pub fn find_by_guild(guild: u64) -> Select<Self> {
        Self::find().filter(Column::GuildId.eq(guild.to_string()))
    }
    pub fn find_by_channel(channel: u64) -> Select<Self> {
        Self::find().filter(Column::ChannelId.eq(channel.to_string()))
    }
    pub fn find_by_role(role: u64) -> Select<Self> {
        Self::find().filter(Column::RoleId.eq(role.to_string()))
    }
    pub fn find_by_id(id: u64) -> Select<Self> {
        Self::find().filter(Column::Id.eq(id.to_string()))
    }
}
