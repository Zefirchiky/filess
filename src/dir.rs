use std::{
    io,
    ops::Div,
    path::{Path, PathBuf},
};

use crate::FileTrait;
pub use crate::{FileType, FsHandler};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dir {
    path: PathBuf,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub files: Vec<FileType>,
}

impl Dir {
    /// Creates a new `Dir` instance from a given path.
    ///
    /// If the path already exists, it must be a directory. If it does not exist, it will be created recursively.
    ///
    /// # Panics
    ///
    /// If the path cannot be created, a panic will occur with the error message.
    pub fn new(dir: impl AsRef<Path>) -> Self {
        let dir = dir.as_ref().to_path_buf();

        if dir.exists() {
            assert!(!dir.is_dir(), "{dir:?} is not a directory")
        }

        Self {
            path: dir,
            files: vec![],
        }
    }

    /// Will create a this directory and recursively create all child files and directories.
    ///
    /// It is recommended to use async version of this method
    pub fn create_all(&self) -> io::Result<()> {
        self.create()?;
        for file in &self.files {
            file.create()?;
        }

        Ok(())
    }

    #[cfg(feature = "async")]
    pub async fn acreate_all(&self) -> io::Result<()> {
        use crate::FileTraitAsync;

        self.acreate().await?;
        for file in &self.files {
            file.acreate().await?;
        }

        Ok(())
    }

    pub fn create(&self) -> io::Result<()> {
        std::fs::create_dir_all(&self)
    }

    #[cfg(feature = "async")]
    pub async fn acreate(&self) -> io::Result<()> {
        tokio::fs::create_dir_all(&self).await
    }

    /// Adds a file to this directory. Path should be relative to the folder
    pub fn add(&mut self, file: FileType) {
        self.files.push(file)
    }
}

impl AsRef<std::path::Path> for Dir {
    fn as_ref(&self) -> &std::path::Path {
        &self.path
    }
}

impl From<&std::path::Path> for Dir {
    fn from(path: &std::path::Path) -> Self {
        Self::new(path)
    }
}

impl From<std::path::PathBuf> for Dir {
    fn from(path: std::path::PathBuf) -> Self {
        Self::new(path)
    }
}

impl From<&str> for Dir {
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}

impl From<String> for Dir {
    fn from(path: String) -> Self {
        Self::new(path)
    }
}

impl std::ops::Deref for Dir {
    type Target = Vec<FileType>;
    fn deref(&self) -> &Self::Target {
        &self.files
    }
}

impl std::ops::DerefMut for Dir {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.files
    }
}

impl Div<Self> for Dir {
    type Output = Self;
    fn div(self, rhs: Dir) -> Self::Output {
        Self::new(self.as_ref().join(rhs))
    }
}

impl Div<&str> for Dir {
    type Output = FsHandler;
    fn div(self, rhs: &str) -> Self::Output {
        let new_path = self.path.join(rhs);
        if let Some(_) = new_path.extension() {
            FsHandler::File(FileType::from_ext(&self.as_ref().join(rhs)))
        } else {
            FsHandler::Dir(Self::new(self.as_ref().join(rhs)))
        }
    }
}

impl IntoIterator for Dir {
    type Item = FileType;
    type IntoIter = <Vec<FileType> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}

impl<'a> IntoIterator for &'a Dir {
    type Item = &'a FileType;
    type IntoIter = <&'a Vec<FileType> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.files.iter()
    }
}

impl<'a> IntoIterator for &'a mut Dir {
    type Item = &'a mut FileType;
    type IntoIter = <&'a mut Vec<FileType> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.files.iter_mut()
    }
}
