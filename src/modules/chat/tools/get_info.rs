use std::sync::Arc;

use async_trait::async_trait;
use llm_rust::{
    embedding::{embedder_trait::Embedder, openai::openai_embedder::OpenAiEmbedder},
    tools::tool_trait::Tool,
};

use crate::errors::Error;

use super::port::ToolRepository;

#[derive(Clone)]
pub struct GetInfoTool {
    name: String,
    description: String,
    organisation_id: i32,
    repo: Arc<dyn ToolRepository>,
}

impl GetInfoTool {
    pub fn new(organisation_id: i32, repo: Arc<dyn ToolRepository>) -> Self {
        Self {
            name: String::from("Get_Info"),
            description: String::from(
                "This tool can get information from a Knolage database, the input is a query",
            ),
            organisation_id,
            repo,
        }
    }

    async fn fetch_embedding(&self, input: &str) -> Result<Vec<f32>, Error> {
        let embedder = OpenAiEmbedder::default();
        Ok(embedder
            .embed_query(input)
            .await
            .map_err(|e| Error::internal_server_error(e.to_string()))?
            .iter()
            .map(|x| *x as f32)
            .collect())
    }
}

#[async_trait]
impl Tool for GetInfoTool {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    async fn call(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let embedding = self.fetch_embedding(input).await?;
        let data = self
            .repo
            .get_data(self.organisation_id, &embedding, 10)
            .await?;
        Ok(data.join("\n"))
    }
}
