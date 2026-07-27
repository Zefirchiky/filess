use std::{fmt::Debug, fs, path::{Path, PathBuf}};

pub trait FsElementBoundries: Debug + Clone + AsRef<Path> + From<PathBuf> + From<&'static str> + Sync + Send {}
impl<F: FsElement> FsElementBoundries for F {}

pub trait FsElement: FsElementBoundries {
    type TryNewError: std::error::Error;
    
    /// Creates new file or dir
    /// 
    /// Panics at error
    fn new(path: impl AsRef<Path>) -> Self {
        Self::try_new(path).unwrap()
    }
    /// Creates new file or dir
    fn try_new(path: impl AsRef<Path>) -> Result<Self, Self::TryNewError>;
    
    /// Creates file or dir in file system
    fn create(&self) -> std::io::Result<()>;
    /// Removes file or dir from file system
    fn remove(&self) -> std::io::Result<()>;
    /// Copies file or dir in file system
    fn copy(&self, path: impl AsRef<Path>) -> std::io::Result<Self> {
        fs::copy(self, &path)?;
        Ok(Self::new(path))
    }
    /// Renames the file or a dir in a file system
    /// Corresponds to `fs::rename`
    fn rename(&self, path: impl AsRef<Path>) -> std::io::Result<Self> {
        fs::rename(self, &path)?;
        Ok(Self::new(path))
    }
    /// Changes underlying `PathBuf`
    /// Different from `Self::rename` in that it does NOT change file or dir in the file system
    fn rename_file(&mut self, name: impl AsRef<Path>);
    /// Moves file in trash
    #[cfg(feature = "trash")]
    fn trash(&self) -> Result<(), trash::Error> {
        trash::delete(self)
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait AsyncFsElement: FsElement {
    /// Creates file or dir in file system
    async fn acreate(&self) -> std::io::Result<()>;
    /// Removes file or dir from file system
    async fn aremove(&self) -> std::io::Result<()>;
    /// Copies file or dir in file system
    async fn acopy(&self, path: impl AsRef<Path> + Sync + Send) -> std::io::Result<Self> {
        tokio::fs::copy(self, &path).await?;
        Ok(Self::new(path))
    }
    /// Renames the file or a dir in a file system
    /// Corresponds to `fs::rename`
    async fn arename(&self, path: impl AsRef<Path> + Sync + Send) -> std::io::Result<Self> {
        tokio::fs::rename(self, &path).await?;
        Ok(Self::new(path))
    }
}

#[cfg(feature = "open")]
impl<F: FsElement> crate::primitives::OpenTrait for F {}
