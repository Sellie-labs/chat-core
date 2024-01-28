use std::io::{self, ErrorKind};

use async_trait::async_trait;
use llm_rust::schemas::messages::{is_base_message, message_from_map, BaseMessage};
use serde_json::Value;

use crate::{
    errors::Error,
    modules::chat::{model::Chat, port::DBRepository},
    utils::infrastructure::postgres_repository::PostgresRepository,
};

#[async_trait]
impl DBRepository for PostgresRepository {
    async fn get_chat_session(&self, identifier: &str) -> Result<Chat, Error> {
        let row = sqlx::query!(
            "SELECT cs.id, cs.organization_id, cs.source, cs.identifier, cs.history, oc.web_prompt, oc.apps_prompt
             FROM Chat_Session cs
             JOIN Organization_Configs oc ON cs.organization_id = oc.organization_id
             WHERE cs.identifier = $1
             LIMIT 1",
            identifier
        )
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch chat session: {}", e);
            Error::internal_server_error(format!("Failed to fetch chat session: {}", e))
        })?;

        let history = match &row.history {
            Some(history_value) => {
                history_value.as_array().map_or_else(
                    || vec![],
                    |history_array| {
                        history_array
                            .iter()
                            .filter_map(|value| {
                                // Attempt to deserialize each `Value` in the array into a `Box<dyn BaseMessage>`
                                serde_json::from_value(value.clone()).ok()
                            })
                            .collect()
                    },
                )
            }
            None => vec![], // If `history` is None, default to an empty vector
        };

        Ok(Chat::new(
            row.id,
            row.organization_id.unwrap_or(0),
            &row.source.unwrap_or_default(),
            &row.identifier.unwrap_or_default(),
            history,
            &row.web_prompt.unwrap_or_default(),
            &row.apps_prompt.unwrap_or_default(),
        ))
    }
}
pub fn message_from_value(
    value: Value,
) -> Result<Box<dyn BaseMessage>, Box<dyn std::error::Error + Send>> {
    if let Value::Object(map) = value {
        let string_map = map
            .into_iter()
            .map(|(k, v)| (k, v.as_str().unwrap_or("").to_string()))
            .collect();
        message_from_map(string_map)
    } else {
        Err(Box::new(io::Error::new(
            ErrorKind::InvalidInput,
            "Expected an object for message deserialization",
        )))
    }
}

// Convert a Vec<serde_json::Value> to Vec<Box<dyn BaseMessage>>
pub fn messages_from_map(
    values: Vec<Value>,
) -> Result<Vec<Box<dyn BaseMessage>>, Box<dyn std::error::Error + Send>> {
    values.into_iter().map(message_from_value).collect()
}
