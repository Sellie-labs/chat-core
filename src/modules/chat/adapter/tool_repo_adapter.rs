use async_trait::async_trait;
use pgvector::Vector;
use sqlx::Row;

use crate::{
    errors::Error, modules::chat::tools::port::ToolRepository,
    utils::infrastructure::postgres_repository::PostgresRepository,
};

#[async_trait]
impl ToolRepository for PostgresRepository {
    async fn get_data(
        &self,
        organization_id: i32,
        question: &[f32],
        limit: i32,
    ) -> Result<Vec<String>, Error> {
        let rows = sqlx::query(
            "SELECT data 
             FROM indexed_data 
             WHERE organization_id = $1
             ORDER BY embedding <-> $2
             LIMIT $3;",
        )
        .bind(organization_id)
        .bind(&Vector::from(question.to_vec()))
        .bind(limit)
        .map(|row| row.get::<String, _>("data"))
        .fetch_all(&*self.pg_pool)
        .await
        .map_err(|e| {
            log::error!("Failed to execute query: {}", e);
            Error::internal_server_error(format!("Failed to execute query: {}", e))
        })?;

        Ok(rows)
    }
}
