use std::path::{Path, PathBuf};

const DICOM_SOURCE_DIRECTORY: &str = "archive";
const DICOM_OUTPUT_DIRECTORY: &str = "dicom_output";
const MASKS_SOURCE_DIRECTORY: &str = "masks";
const NIFTY_SOURCE_DIRECTORY: &str = "nifty";
const NIFTY_OUTPUT_DIRECTORY: &str = "nifty_output";

#[derive(Debug, thiserror::Error)]
pub enum FileManagerError {
    #[error("Failed to canonicalize {0}")]
    CanonicalizeError(#[source] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct Directory {
    path: PathBuf,
}

impl Directory {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Directory { path: path.into() }
    }

    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn absolute(&self) -> Result<PathBuf, FileManagerError> {
        Ok(std::fs::canonicalize(self.as_path())
            .map_err(FileManagerError::CanonicalizeError)?
            .clone())
    }

    pub fn create_if_not_exists(&self) -> Result<(), FileManagerError> {
        todo!()
    }

    pub fn list_files(&self) -> Vec<String> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct FileManager {
    root: Directory,
}

impl FileManager {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        let name = ulid::Ulid::new().to_string();
        let path = root.into().as_path().join(name);

        Self {
            root: Directory::new(&path),
        }
    }

    pub fn create(&self) -> Result<(), FileManagerError> {
        self.root.create_if_not_exists()?;
        self.dicom_output().create_if_not_exists()?;
        self.dicom_source().create_if_not_exists()?;
        self.masks_output().create_if_not_exists()?;
        self.nifty_output().create_if_not_exists()?;
        self.nifty_source().create_if_not_exists()?;
        Ok(())
    }

    pub fn dicom_output(&self) -> Directory {
        self.subdir_from_root(DICOM_OUTPUT_DIRECTORY)
    }

    pub fn dicom_source(&self) -> Directory {
        self.subdir_from_root(DICOM_SOURCE_DIRECTORY)
    }

    pub fn masks_output(&self) -> Directory {
        self.subdir_from_root(MASKS_SOURCE_DIRECTORY)
    }

    pub fn nifty_output(&self) -> Directory {
        self.subdir_from_root(NIFTY_OUTPUT_DIRECTORY)
    }

    pub fn nifty_source(&self) -> Directory {
        self.subdir_from_root(NIFTY_SOURCE_DIRECTORY)
    }

    fn subdir_from_root(&self, subdir: &str) -> Directory {
        Directory::new(self.root.as_path().join(subdir))
    }
}
