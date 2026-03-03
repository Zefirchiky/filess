use std::{
    io,
    ops::Div,
    path::{Path, PathBuf},
};

use derive_more::{AsMut, AsRef, Deref, DerefMut, From, IntoIterator};

use crate::FileTrait;
pub use crate::{FileType, FsHandler};

#[derive(Debug, Default, From, IntoIterator, AsRef, AsMut, Deref, DerefMut)]
#[from(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dir {
    #[deref]
    #[deref_mut]
    #[as_ref(forward)]
    path: PathBuf,
    #[into_iterator(owned, ref, ref_mut)]
    #[from(skip)]
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

impl From<&Path> for Dir {
    fn from(value: &Path) -> Self {
        Self::new(value)
    }
}

impl From<PathBuf> for Dir {
    fn from(value: PathBuf) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl<P: crate::FileTrait> Div<P> for Dir {
    type Output = P;
    fn div(self, rhs: P) -> Self::Output {
        P::from(self.join(rhs))
    }
}

impl Div<Self> for Dir {
    type Output = Self;
    fn div(self, rhs: Dir) -> Self::Output {
        Self::new(self.join(rhs))
    }
}

impl Div<&str> for Dir {
    type Output = FsHandler;
    fn div(self, rhs: &str) -> Self::Output {
        let new_path = self.path.join(rhs);
        if let Some(_) = new_path.extension() {
            FsHandler::File(FileType::from_ext(&self.join(rhs)))
        } else {
            FsHandler::Dir(Self::new(self.join(rhs)))
        }
    }
}
