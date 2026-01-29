use crate::domain::error::AppError;

pub struct ExtractionService;

impl ExtractionService {
    pub fn new() -> Self {
        Self
    }

    pub async fn extract_text(&self, _file_path: &str) -> Result<String, AppError> {
        Ok(String::new())
    }
}
