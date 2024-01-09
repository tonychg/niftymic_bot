use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    #[error("Failed to create new archive: {0}")]
    CreateError(#[source] zip::result::ZipError),
    #[error("Failed to extract archive: {0}")]
    ExtractError(#[source] zip::result::ZipError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub struct Archive {
    path: PathBuf,
}

impl Archive {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Archive { path: path.into() }
    }

    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn create<P: AsRef<Path>>(&self, paths: &[P]) -> Result<(), ArchiveError> {
        let file = File::create(self.as_path())?;
        let mut zip = zip::ZipWriter::new(file);
        for path in paths {
            let buffer = std::fs::read(&path)?;
            zip.start_file(
                // We can unwrap safely here, because we know that the path is a file and is UTF-8 encoded.
                path.as_ref().file_name().unwrap().to_str().unwrap(),
                zip::write::FileOptions::default(),
            )
            .map_err(ArchiveError::CreateError)?;
            zip.write_all(&buffer)?;
        }
        Ok(())
    }

    pub fn extract<P: AsRef<Path>>(&self, destination: P) -> Result<(), ArchiveError> {
        let file = File::open(self.as_path())?;
        let mut archive = zip::ZipArchive::new(file).map_err(ArchiveError::ExtractError)?;
        Ok(archive
            .extract(destination)
            .map_err(ArchiveError::ExtractError)?)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_archive_new_from_path() {
        let path = Path::new("test.zip");
        let archive = Archive::new(path);
        assert_eq!(archive.as_path(), path);
    }

    #[test]
    fn test_archive_new_from_str() {
        let archive = Archive::new("test.zip");
        assert_eq!(archive.as_path(), Path::new("test.zip"));
    }

    #[test]
    fn test_archive_create() {
        let target = tempdir().unwrap();
        let dest = tempdir().unwrap();
        let file_path = target.path().join("test01.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test01").unwrap();
        let archive = Archive::new(dest.path().join("test.zip"));
        let paths = vec![file_path];
        archive.create(&paths).unwrap();
    }

    #[test]
    fn test_archive_create_with_invalid_path() {
        let archive = Archive::new("/invalid/path/test.zip");
        assert!(archive.create::<PathBuf>(&[]).is_err());
    }
}
