use async_trait::async_trait;

use crate::errors::Error;

use super::model::Chat;

#[async_trait]
pub trait DBRepository: Send + Sync {
    async fn get_chat_session(&self, identifier: &str) -> Result<Chat, Error>;
}
