use crate::models::MaybePartialUser;
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

/// Represents a bot account.
#[derive(Clone, Debug, Default, Serialize)]
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub struct Bot {
    /// The bot user.
    #[serde(flatten)]
    pub user: MaybePartialUser,
    /// The owner ID of the bot.
    pub owner_id: u64,
}