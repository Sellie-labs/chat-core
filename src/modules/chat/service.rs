use std::sync::Arc;

use llm_rust::{
    agents::{agent::Agent, chat::ConversationalAgentBuilder, executor::AgentExecutor},
    chains::chain_trait::ChainTrait,
    chat_models::openai::{ChatModel, ChatOpenAI},
    schemas::{chain::ChainResponse, memory::BaseChatMessageHistory},
    tools::tool_trait::Tool,
};
use tokio::sync::RwLock;

use crate::errors::Error;

use super::{
    model::ChatHistory,
    port::DBRepository,
    tools::{get_info::GetInfoTool, port::ToolRepository},
};

pub struct Service {
    repo: Arc<dyn DBRepository>,
    tool_repo: Arc<dyn ToolRepository>,
}

impl Service {
    pub fn new(repo: Arc<dyn DBRepository>, tool_repo: Arc<dyn ToolRepository>) -> Self {
        Self { repo, tool_repo }
    }

    pub async fn chat(&self, question: &str, chat_id: &str) -> Result<String, Error> {
        let chat_session = self.repo.get_chat_session(chat_id).await?;
        let prompt = {
            if &chat_session.source == "widget" {
                chat_session.web_prompt
            } else {
                chat_session.apps_prompt
            }
        };

        let agent = self.build_agent(chat_session.organization_id, &prompt)?;
        let history = Arc::new(RwLock::new(ChatHistory::from(chat_session.history)));

        let executor = AgentExecutor::from_agent(agent)
            .with_memory(Arc::clone(&history) as Arc<RwLock<dyn BaseChatMessageHistory>>);

        let resp = executor.run(&question.to_string()).await.map_err(|e| {
            log::error!("Failed to generate chat response: {}", e.to_string());
            Error::internal_server_error(e.to_string())
        })?;

        match resp {
            ChainResponse::Text(message) => {
                log::info!("Response: {}", message);
                return Ok(message);
            }
            _ => {
                return Err(Error::internal_server_error(
                    "Invalid response type".to_string(),
                ));
            }
        }
    }

    fn build_agent(&self, organization_id: i32, prompt: &str) -> Result<Box<dyn Agent>, Error> {
        let tools = {
            let get_info = Arc::new(GetInfoTool::new(organization_id, self.tool_repo.clone()))
                as Arc<dyn Tool>;
            vec![get_info]
        };

        let llm = ChatOpenAI::default()
            .with_model(ChatModel::Gpt4)
            .with_temperature(0.0);

        let agent = ConversationalAgentBuilder::new()
            .tools(tools)
            .llm(Box::new(llm))
            .prefix(prompt)
            .build()
            .map_err(|e| {
                log::error!("Failed to build agent: {}", e.to_string());
                Error::internal_server_error(e.to_string())
            })?;

        Ok(Box::new(agent))
    }
}
