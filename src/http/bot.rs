use crate::Maybe;
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

/// Payload sent to create a new bot.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct CreateBotPayload {
    /// The username of the bot. Must be between 2 and 32 characters.
    pub username: String,
}

/// Data returned when creating a new bot.
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct CreateUserResponse {
    /// The ID of the bot.
    pub id: u64,
    /// The token to use for authentication.
    pub token: String,
}

/// Payload sent when deleting a bot.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct DeleteBotPayload {
    /// The password of the bot owner.
    #[cfg_attr(feature = "openapi", schema(format = "password"))]
    pub password: String,
}