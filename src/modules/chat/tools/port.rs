use async_trait::async_trait;

use crate::errors::Error;

#[async_trait]
pub trait ToolRepository: Send + Sync {
    async fn get_data(
        &self,
        organization_id: i32,
        question: &[f32],
        limit: i32,
    ) -> Result<Vec<String>, Error>;
}
