use std::{
    ffi::OsString, fmt::Debug, fs::{self, create_dir_all}, marker::PhantomData, ops::{Deref, DerefMut}, path::{Path, PathBuf},
};

use crate::traits::FsElement;

#[derive(Debug, thiserror::Error)]
pub enum FileCreationError<F: FileTrait> {
    #[error("Extension should be a valid UTF-8")]
    InvalidUtf8(OsString),
    #[error("Extension must be one of `{ext:?}` for file {0:?}, given: `{1}`", ext = F::ext())]
    WrongExtension(PathBuf, String),
    #[error("Extension must be one of `{ext:?}` for file {0:?}, no extension given", ext = F::ext())]
    NoExtension(PathBuf),
    #[error("Should be unreachable")]
    _Phantom(F)
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FileBase<F: FileTrait> {
    // TODO: With thousands of paths, central storage is preferable. Something like a mini filesystem. OPTIMIZATIONS BABE
    pub path: PathBuf,
    _phantom: PhantomData<F>,
}

impl<F: FileTrait> FileBase<F> {
    /// Creates a new FileHandler.
    ///
    /// # Panics
    ///
    /// Panics if the path is not a file or if the file does not have the correct extension.
    pub fn new(file: impl AsRef<Path>) -> Self {
        Self::try_new(file).unwrap()
    }
    
    /// Creates a new FileHandler.
    ///
    /// # Panics
    ///
    /// Panics if the path is not a file or if the file does not have the correct extension.
    pub fn try_new(file: impl AsRef<Path>) -> Result<Self, F::TryNewError> {
        let file = file.as_ref().to_path_buf();

        if !F::ext().is_empty() {
            match file.extension() {
                Some(ext) => {
                    let ext = ext.to_str().ok_or(F::TryNewError::InvalidUtf8(ext.to_owned()))?;
                    if !F::ext().contains(&ext) {
                        return Err(F::TryNewError::WrongExtension(file.clone(), ext.into()));
                    }
                }
                None => {
                    return Err(F::TryNewError::NoExtension(file.clone()));
                }
            }
        }

        Ok(Self {
            path: file,
            _phantom: PhantomData,
        })
    }
}

#[cfg(feature = "open")]
impl<F: FileTrait> crate::traits::OpenTrait for FileBase<F> {}

impl<H: FileTrait> AsRef<Path> for FileBase<H> {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl<H: FileTrait> AsMut<Path> for FileBase<H> {
    fn as_mut(&mut self) -> &mut Path {
        &mut self.path
    }
}

impl<H: FileTrait> From<&Path> for FileBase<H> {
    fn from(path: &Path) -> Self {
        Self::new(path)
    }
}

impl<H: FileTrait> From<PathBuf> for FileBase<H> {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl<H: FileTrait> From<&str> for FileBase<H> {
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}

impl<H: FileTrait> From<String> for FileBase<H> {
    fn from(path: String) -> Self {
        Self::new(path)
    }
}

impl<H: FileTrait> Deref for FileBase<H> {
    type Target = PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl<H: FileTrait> DerefMut for FileBase<H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}

pub trait FileTrait: FsElement<TryNewError = FileCreationError<Self>> + Default {
    fn new(path: impl AsRef<Path>) -> Self {
        <Self as FileTrait>::try_new(path).unwrap()
    }
    fn try_new(path: impl AsRef<Path>) -> Result<Self, Self::TryNewError>;
    
    fn change_path(&mut self, path: PathBuf);
    /// Initial file bytes, if needed
    fn file_init_bytes() -> Option<&'static [u8]> {
        None
    }
    /// Possible file extension that will be forced
    fn ext() -> &'static [&'static str];
    fn ext_name() -> &'static str;
    fn mime_type() -> &'static [&'static str];

    /// Returns `std::fs::File` for this `File`
    fn as_file(&self) -> std::io::Result<fs::File> {
        fs::File::create(self)
    }

    /// Saves data to the file.
    ///
    /// File will be created if it didn't exist.
    fn save(&self, data: impl AsRef<[u8]>) -> std::io::Result<()> {
        if let Some(parent) = self.as_ref().parent() {
            create_dir_all(parent)?
        }
        fs::write(self.as_ref(), data)?;
        Ok(())
    }

    /// Loads data from a file.
    ///
    /// If file didn't exist, it will be created and `F::file_init_bytes()` will be returned.
    fn load(&self) -> std::io::Result<Vec<u8>> {
        if !self.as_ref().try_exists()? {
            self.create()?;
        }
        fs::read(self.as_ref())
    }

    /// Opens file in default program using `open::that_detached()`
    ///
    /// For other methods use `open` crate directly with `file.as_ref()`
    #[cfg(feature = "open")]
    fn open(&self) -> std::io::Result<()> {
        open::that_detached(self.as_ref())
    }

    #[cfg(feature = "infer")]
    fn infer(&self) -> std::io::Result<Option<infer::Type>> {
        Ok(infer::get(&self.load()?))
    }

    #[cfg(feature = "infer")]
    fn is_correct_data(&self) -> std::io::Result<bool> {
        if let Some(t) = self.infer()? {
            Ok(Self::ext().contains(&t.extension()))
        } else {
            Ok(false)
        }
    }

    /// Enforces file data to be of file type.
    ///
    /// It's an io operation, use `aenforce` if you don't want this operation to lag the program.
    #[cfg(feature = "infer")]
    fn enforce(&self) -> std::io::Result<()> {
        assert!(
            self.is_correct_data()?,
            "{:?} contains incorrect data. Inferred data type: {:?}",
            self,
            self.infer().unwrap()
        );
        Ok(())
    }
}

impl<T: FileTrait> FsElement for T {
    type TryNewError = FileCreationError<T>;
    
    fn try_new(path: impl AsRef<Path>) -> Result<Self, Self::TryNewError> {
        <T as FileTrait>::try_new(path)
    }

    /// Creates a new file.
    ///
    /// !!! OVERWRITES CONTENT IF FILE ALREADY EXISTS !!!
    fn create(&self) -> std::io::Result<()> {
        if let Some(parent) = self.as_ref().parent() {
            create_dir_all(parent)?
        }

        match Self::file_init_bytes() {
            Some(b) => fs::write(self, b)?,
            None => {
                fs::File::create(self)?;
            }
        };

        Ok(())
    }

    /// Removes the file from the disk
    fn remove(&self) -> std::io::Result<()> {
        fs::remove_file(self)
    }

    /// Corresponds to `fs::copy`.
    ///
    /// Copies file to the new path.
    /// Does not consume this file.
    /// New file instance will be returned.
    fn copy(&self, path: impl AsRef<Path>) -> std::io::Result<Self> {
        fs::copy(self, &path)?;
        Ok(<Self as FileTrait>::new(path))
    }
    
    /// Renames the file in a file system.
    /// 
    /// Corresponds to `fs::rename`.
    fn rename(&self, path: impl AsRef<Path>) -> std::io::Result<Self> {
        fs::rename(self, &path)?;
        Ok(<Self as FileTrait>::new(path))
    }

    /// Changes underlying `PathBuf`
    /// 
    /// Different from `Self::rename` in that it does NOT change file or dir in the file system
    fn rename_file(&mut self, name: impl AsRef<Path>) {
        self.change_path(name.as_ref().into());
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl<T: FileTrait> crate::traits::AsyncFsElement for T {
    async fn acreate(&self) -> std::io::Result<()> {
        use tokio::fs;

        if let Some(parent) = self.as_ref().parent() {
            fs::create_dir_all(parent).await?
        }

        match Self::file_init_bytes() {
            Some(b) => fs::write(self, b).await?,
            None => {
                fs::File::create(self).await?;
            }
        }

        Ok(())
    }

    async fn aremove(&self) -> std::io::Result<()> {
        tokio::fs::remove_file(self).await
    }
}

#[cfg(feature = "async")]
impl<T: FileTrait + crate::traits::AsyncFsElement> AsyncFileTrait for T {}

#[cfg(feature = "async")]
pub trait AsyncFileTrait: FileTrait + crate::traits::AsyncFsElement {
    async fn asave(&self, data: impl AsRef<[u8]>) -> std::io::Result<()> {
        if let Some(parent) = self.as_ref().parent() {
            tokio::fs::create_dir_all(parent).await?
        }
        tokio::fs::write(&self.as_ref(), data).await?;
        Ok(())
    }

    async fn aload(&self) -> std::io::Result<Vec<u8>> {
        if !tokio::fs::try_exists(self).await? {
            self.acreate().await?;
        }
        tokio::fs::read(&self.as_ref()).await
    }

    #[cfg(feature = "infer")]
    async fn ainfer(&self) -> std::io::Result<Option<infer::Type>> {
        Ok(infer::get(&self.aload().await?))
    }

    #[cfg(feature = "infer")]
    async fn ais_correct_data(&self) -> std::io::Result<bool> {
        if let Some(t) = self.ainfer().await? {
            Ok(Self::ext().contains(&t.extension()))
        } else {
            Ok(false)
        }
    }

    /// Enforces file data to be of file type
    #[cfg(feature = "infer")]
    async fn aenforce(&self) -> std::io::Result<()> {
        assert!(
            self.ais_correct_data().await?,
            "{:?} contains incorrect data. Inferred data type: {:?}",
            self,
            self.ainfer().await.unwrap()
        );
        Ok(())
    }
}

