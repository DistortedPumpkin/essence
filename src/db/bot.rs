use crate::{
    db::{get_pool, user::construct_user},
    models::user::{User, UserFlags},
    Error
};

#[async_trait::async_trait]
pub trait BotDbExt<'t>: DbExt<'t> {
    /// Registers a bot in the database with the given payload. No validation is done; they must
    /// be done before calling this method.
    ///
    /// # Note
    /// This method uses transactions, on the event of an ``Err`` the transaction must be properly
    /// rolled back, and the transaction must be committed to save the changes.
    ///
    /// # Errors
    /// * If an error occurs with registering the bot.
    #[cfg(feature = "auth")]
    async fn register_bot(
        &mut self,
        id: u64,
        owner_id: u64,
        username: impl AsRef<str> + Send,
        
    ) -> crate::Result<()> {
        let discriminator = sqlx::query!(
            "INSERT INTO
                users (id, username, flags)
            VALUES
                ($1, $2, $3)
            RETURNING
                discriminator",
            id as i64,
            username.as_ref().trim(),
            UserFlags::BOT,
        )
        .fetch_optional(self.transaction())
        .await?;

        if discriminator.is_none() {
            return Err(Error::AlreadyTaken {
                what: "username".to_string(),
                message: "Username is already taken".to_string(),
            });
        }

        sqlx::query!(
            "INSERT INTO bots VALUES ($1, $2)",
            id as i64,
            owner_id as i64,
        )
        .execute(self.transaction())
        .await?;

        Ok(())
    }

}
impl<'t, T> BotDbExt<'t> for T where T: DbExt<'t> {}