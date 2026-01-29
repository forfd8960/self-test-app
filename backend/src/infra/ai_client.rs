use async_openai::{
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use thiserror::Error;

use crate::config::AppConfig;

#[derive(Debug, Error)]
pub enum AiClientError {
    #[error("OpenAI error: {0}")]
    OpenAi(#[from] async_openai::error::OpenAIError),
    #[error("unexpected response")]
    UnexpectedResponse,
}

#[derive(Clone)]
pub struct AiClient {
    client: Client<OpenAIConfig>,
    model: String,
}

impl AiClient {
    pub fn new(config: &AppConfig) -> Self {
        let openai_config = OpenAIConfig::new()
            .with_api_base(&config.ai_base_url)
            .with_api_key(&config.ai_api_key);
            
        Self {
            client: Client::with_config(openai_config),
            model: config.ai_model.clone(),
        }
    }

    pub async fn generate(&self, prompt: &str) -> Result<String, AiClientError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages([ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into()])
            .build()?;

        let response = self.client.chat().create(request).await?;

        response.choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or(AiClientError::UnexpectedResponse)
    }
}
