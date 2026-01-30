use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::domain::error::AppError;

type ExtractorFn = fn(PathBuf) -> Result<String, AppError>;

pub struct ExtractionService {
    extractors: HashMap<String, ExtractorFn>,
}

impl ExtractionService {
    pub fn new() -> Self {
        Self::with_default_extractors()
    }

    pub fn with_default_extractors() -> Self {
        let mut extractors: HashMap<String, ExtractorFn> = HashMap::new();
        extractors.insert("txt".to_string(), extract_txt as ExtractorFn);
        extractors.insert("pdf".to_string(), extract_pdf as ExtractorFn);
        extractors.insert("docx".to_string(), extract_docx as ExtractorFn);
        Self { extractors }
    }

    pub fn register_extractor(&mut self, file_type: &str, extractor: ExtractorFn) {
        self.extractors.insert(file_type.to_lowercase(), extractor);
    }

    pub fn is_supported(&self, file_type: &str) -> bool {
        self.extractors.contains_key(&file_type.to_lowercase())
    }

    pub async fn extract_text(
        &self,
        file_type: &str,
        file_path: &Path,
    ) -> Result<Option<String>, AppError> {
        let extractor = match self.extractors.get(&file_type.to_lowercase()) {
            Some(extractor) => *extractor,
            None => return Ok(None),
        };

        let path = file_path.to_path_buf();
        let text = tokio::task::spawn_blocking(move || extractor(path))
            .await
            .map_err(|e| {
                tracing::error!("Extraction task failed: {:?}", e);
                AppError::Internal
            })??;

        Ok(Some(text))
    }
}

fn extract_txt(path: PathBuf) -> Result<String, AppError> {
    let bytes = std::fs::read(&path).map_err(|e| {
        tracing::error!("Failed to read txt file {:?}: {:?}", path, e);
        AppError::Internal
    })?;

    String::from_utf8(bytes).map_err(|e| {
        tracing::error!("Invalid UTF-8 in txt file {:?}: {:?}", path, e);
        AppError::Internal
    })
}

fn extract_pdf(path: PathBuf) -> Result<String, AppError> {
    pdf_extract::extract_text(&path).map_err(|e| {
        tracing::error!("PDF extraction failed for {:?}: {:?}", path, e);
        AppError::Internal
    })
}

fn extract_docx(path: PathBuf) -> Result<String, AppError> {
    let file = File::open(&path).map_err(|e| {
        tracing::error!("Failed to open docx {:?}: {:?}", path, e);
        AppError::Internal
    })?;

    let mut archive = zip::ZipArchive::new(file).map_err(|e| {
        tracing::error!("Failed to read docx zip {:?}: {:?}", path, e);
        AppError::Internal
    })?;

    let mut document = archive.by_name("word/document.xml").map_err(|e| {
        tracing::error!("Missing document.xml in {:?}: {:?}", path, e);
        AppError::Internal
    })?;

    let mut xml = String::new();
    document.read_to_string(&mut xml).map_err(|e| {
        tracing::error!("Failed to read document.xml in {:?}: {:?}", path, e);
        AppError::Internal
    })?;

    let mut reader = quick_xml::Reader::from_str(&xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut text = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Text(e)) => {
                let value = e.unescape().map_err(|e| {
                    tracing::error!("Failed to unescape docx text: {:?}", e);
                    AppError::Internal
                })?;
                if !value.is_empty() {
                    if !text.is_empty() {
                        text.push(' ');
                    }
                    text.push_str(&value);
                }
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => {
                tracing::error!("Failed to parse document.xml in {:?}: {:?}", path, e);
                return Err(AppError::Internal);
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(text)
}
