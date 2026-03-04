use std::{
    fmt::Debug,
    fs::{self, create_dir_all},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use derive_more::{AsRef, Deref, DerefMut};

#[cfg(feature = "async")]
pub use crate::FileTraitAsync as _;
pub use FileTrait as _;

#[derive(Debug, Clone, Default, AsRef, Deref, DerefMut, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FileBase<F: FileTrait> {
    // TODO: With thousands of paths, central storage is preferable. Something like a mini filesystem. OPTIMIZATIONS BABE
    #[as_ref(forward)]
    #[deref]
    #[deref_mut]
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
        let file = file.as_ref().to_path_buf();

        if !F::ext().is_empty() {
            match file.extension() {
                Some(ext) => {
                    let ext = ext.to_str().expect("Extension should be a valid UTF-8");
                    assert!(
                        F::ext().contains(&ext),
                        "Extension must be one of `{:?}` for file {file:?}, given: `{ext}`",
                        F::ext(),
                    )
                }
                None => {
                    panic!(
                        "Extension must be one of `{:?}` for file {file:?}, no extension given",
                        F::ext(),
                    )
                }
            }
        }

        Self {
            path: file,
            _phantom: PhantomData,
        }
    }
}

impl<H: FileTrait> From<&'static Path> for FileBase<H> {
    fn from(path: &'static Path) -> Self {
        Self::new(path)
    }
}

impl<H: FileTrait> From<PathBuf> for FileBase<H> {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl<H: FileTrait> From<&'static str> for FileBase<H> {
    fn from(path: &'static str) -> Self {
        Self::new(path)
    }
}

impl<H: FileTrait> From<String> for FileBase<H> {
    fn from(path: String) -> Self {
        Self::new(path)
    }
}

pub trait FileTrait:
    Debug + Clone + Default + From<PathBuf> + From<&'static str> + AsRef<Path>
{
    /// Creates new file
    fn new(path: impl AsRef<Path>) -> Self;
    /// Initial file bytes, if needed
    fn file_init_bytes() -> Option<&'static [u8]> {
        None
    }
    /// Possible file extension that will be forced
    fn ext() -> &'static [&'static str];

    /// Returns `std::fs::File` for this `File`
    fn as_file(&self) -> std::io::Result<fs::File> {
        fs::File::create(self)
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

    /// Saves data to the file.
    ///
    /// File will be created if it didn't exist.
    fn save(&self, data: &impl AsRef<[u8]>) -> std::io::Result<()> {
        if let Some(parent) = self.as_ref().parent() {
            create_dir_all(parent)?
        }
        fs::write(&self.as_ref(), data)?;
        Ok(())
    }

    /// Loads data from a file.
    ///
    /// If file didn't exist, it will be created and `F::file_init_bytes()` will be returned.
    fn load(&self) -> std::io::Result<Vec<u8>> {
        if !self.as_ref().try_exists()? {
            self.create()?;
        }
        fs::read(&self.as_ref())
    }

    /// Removes the file from the disk
    fn remove(&self) -> std::io::Result<()> {
        fs::remove_file(self)
    }
}

#[cfg(feature = "async")]
pub trait FileTraitAsync: FileTrait {
    async fn acreate(&self) -> std::io::Result<()> {
        use tokio::fs;

        if let Some(parent) = self.as_ref().parent() {
            fs::create_dir_all(parent).await?
        }

        match Self::file_init_bytes() {
            Some(b) => fs::write(&self, b).await?,
            None => {
                fs::File::create(&self).await?;
            }
        }

        Ok(())
    }

    async fn asave(&self, data: &impl AsRef<[u8]>) -> std::io::Result<()> {
        use tokio::fs;
        if let Some(parent) = self.as_ref().parent() {
            fs::create_dir_all(parent).await?
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

    async fn aremove(&self) -> std::io::Result<()> {
        tokio::fs::remove_file(self).await
    }
}

#[cfg(feature = "async")]
impl<T: FileTrait> FileTraitAsync for T {}
