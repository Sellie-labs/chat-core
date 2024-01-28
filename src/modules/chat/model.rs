use llm_rust::schemas::{memory::BaseChatMessageHistory, messages::BaseMessage};

pub struct Chat {
    pub id: i32,
    pub organization_id: i32,
    pub source: String,
    pub identifier: String,
    pub history: Vec<Box<dyn BaseMessage>>,
    pub web_prompt: String,
    pub apps_prompt: String,
}

impl Chat {
    pub fn new(
        id: i32,
        organization_id: i32,
        source: &str,
        identifier: &str,
        history: Vec<Box<dyn BaseMessage>>,
        web_prompt: &str,
        apps_prompt: &str,
    ) -> Self {
        Self {
            id,
            organization_id,
            source: String::from(source),
            identifier: String::from(identifier),
            history,
            web_prompt: String::from(web_prompt),
            apps_prompt: String::from(apps_prompt),
        }
    }
}

#[derive(Clone)]
pub struct ChatHistory(pub Vec<Box<dyn BaseMessage>>);
impl ChatHistory {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn from(messages: Vec<Box<dyn BaseMessage>>) -> Self {
        Self(messages)
    }
}
impl BaseChatMessageHistory for ChatHistory {
    fn messages(&self) -> Vec<Box<dyn BaseMessage>> {
        self.0.clone()
    }

    fn add_message(&mut self, message: Box<dyn BaseMessage>) {
        self.0.push(message);
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
