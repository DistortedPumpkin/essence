use crate::models::PartialGuild;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[cfg(feature = "client")]
use serde::Deserialize;

/// A model representing an invite to a guild.
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub struct Invite {
    /// The code of the invite.
    pub code: String,
    /// The ID of the user that created this invite.
    pub inviter_id: u64,
    /// Partial guild information about the guild this invite leads to. This is `None` when this is
    /// already fetched from a guild.
    pub guild: Option<PartialGuild>,
    /// The ID of the guild this invite leads to.
    pub guild_id: u64,
    /// The ID of the channel this invite leads to, or `None` if this invite leads to no channel.
    pub channel_id: Option<u64>,
    /// A timestamp representing when this invite was created.
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub created_at: DateTime<Utc>,
    /// How many times this invite has been used.
    pub uses: u32,
    /// How many times this invite can be used. ``0`` if unlimited.
    pub max_uses: u32,
    /// How long this invite is valid for, in seconds. ``0`` if this invite never expires. This
    /// counts from the time the invite was created (see `created_at`).
    pub max_age: u32,
}
