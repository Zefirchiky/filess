use std::{fmt::Debug, fs, path::{Path, PathBuf}};

use crate::Temporary;

/// Super-trait bundling the bounds required by [FsElement].
#[cfg(not(feature = "serde"))]
pub trait FsElementBoundaries: Default + Debug + Clone + AsRef<Path> + From<PathBuf> + From<&'static str> + Sync + Send {}
#[cfg(not(feature = "serde"))]
impl<F: FsElement> FsElementBoundaries for F {}
#[cfg(feature = "serde")]
pub trait FsElementBoundaries: Default + Debug + Clone + AsRef<Path> + From<PathBuf> + From<&'static str> + Sync + Send + serde::Serialize + for<'de> serde::Deserialize<'de> {}
#[cfg(feature = "serde")]
impl<F: FsElement> FsElementBoundaries for F {}

/// Common operations for files and directories.
pub trait FsElement: FsElementBoundaries {
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
    
    /// Copies file or dir to the new path.
    /// 
    /// Corresponds to [std::fs::copy]
    /// 
    /// Does not consume this instance.
    /// New instance will be returned.
    fn copy(&self, dst: impl AsRef<Path>) -> std::io::Result<Self> {
        fs::copy(self, &dst)?;
        Ok(Self::new(dst))
    }
    
    /// Renames the file or dir in a file system
    /// 
    /// Corresponds to [fs::rename]
    fn rename(&mut self, dst: impl AsRef<Path>) -> std::io::Result<()> {
        fs::rename(&self, &dst)?;
        self.rename_file(dst);
        Ok(())
    }
    /// Changes underlying [PathBuf]
    /// Different from [Self::rename] in that it does NOT change file or dir in the file system
    fn rename_file(&mut self, name: impl AsRef<Path>);
    
    /// Moves file in trash
    #[cfg(feature = "trash")]
    fn trash(&self) -> Result<(), trash::Error> {
        trash::delete(self)
    }

    /// Transforms file into `Temporary`
    /// 
    /// File will be deleted at drop
    fn as_temp(self) -> Temporary<Self> {
        Temporary::new(self)
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
/// Async counterpart of [FsElement].
pub trait AsyncFsElement: FsElement {
    /// Creates file or dir in file system
    async fn acreate(&self) -> std::io::Result<()>;
    /// Removes file or dir from file system
    async fn aremove(&self) -> std::io::Result<()>;
    /// Copies file or dir in file system
    async fn acopy(&self, dst: impl AsRef<Path> + Sync + Send) -> std::io::Result<Self> {
        tokio::fs::copy(self, &dst).await?;
        Ok(Self::new(dst))
    }
    /// Renames the file or a dir in a file system
    /// Corresponds to [fs::rename]
    async fn arename(&self, dst: impl AsRef<Path> + Sync + Send) -> std::io::Result<Self> {
        tokio::fs::rename(self, &dst).await?;
        Ok(Self::new(dst))
    }
}

#[cfg(feature = "open")]
impl<F: FsElement> crate::traits::OpenTrait for F {}
