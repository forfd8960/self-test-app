use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct FileStorage {
    base_dir: PathBuf,
}

impl FileStorage {
    pub fn new<P: Into<PathBuf>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.into(),
        }
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    pub fn material_path(&self, material_id: &str, extension: &str) -> PathBuf {
        let filename = format!("{}.{}", material_id, extension);
        self.base_dir.join(filename)
    }
}
