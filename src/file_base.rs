use std::{
    fmt::Debug, fs::{self, create_dir_all}, marker::PhantomData, ops::{Deref, DerefMut}, path::{Path, PathBuf}
};

use derive_more::{AsRef, Deref, DerefMut};

#[derive(Debug, Clone, Default, AsRef, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(test, derive(PartialEq))]
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

        Self { path: file, _phantom: PhantomData }
    }
    
    pub fn as_file(&self) -> std::io::Result<fs::File> {
        fs::File::create(self)
    }
    
    /// Creates a new file.
    /// 
    /// !!! OVERWRITES CONTENT IF FILE ALREADY EXISTS !!!
    pub fn create(&self) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            create_dir_all(parent)?
        }
        
        match F::file_init_bytes() {
            Some(b) => fs::write(self, b)?,
            None => { fs::File::create(self)?; },
        };
        
        Ok(())
    }
    
    #[cfg(feature = "async")]
    pub async fn create_async(&self) -> std::io::Result<()> {
        use tokio::fs;

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).await?
        }
        
        match F::file_init_bytes() {
            Some(b) => fs::write(&self, b).await?,
            None => { fs::File::create(&self).await?; }
        }
        
        Ok(())
    }
    
    /// Saves data to the file.
    /// 
    /// File will be created if it didn't exist.
    pub fn save(&self, data: &impl AsRef<[u8]>) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            create_dir_all(parent)?
        }
        fs::write(&self.path, data)?;
        Ok(())
    }

    #[cfg(feature = "async")]
    pub async fn save_async(&self, data: &impl AsRef<[u8]>) -> std::io::Result<()> {
        use tokio::fs;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).await?
        }
        tokio::fs::write(&self.path, data).await?;
        Ok(())
    }
    
    /// Loads data from a file.
    /// 
    /// If file didn't exist, it will be created and `F::file_init_bytes()` will be returned.
    pub fn load(&self) -> std::io::Result<Vec<u8>> {
        if !self.path.try_exists()? { self.create()?; }
        fs::read(&self.path)
    }

    #[cfg(feature = "async")]
    pub async fn load_async(&self) -> std::io::Result<Vec<u8>> {
        if !tokio::fs::try_exists(self).await? { self.create_async().await?; }
        tokio::fs::read(&self.path).await
    }
    
    pub fn remove(&self) -> std::io::Result<()> {
        fs::remove_file(self)
    }
    
    #[cfg(feature = "async")]
    pub async fn remove_async(&self) -> std::io::Result<()> {
        tokio::fs::remove_file(self).await
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
    Debug
    + Clone
    + Default
    + From<PathBuf>
    + From<&'static str>
    + AsRef<Path>
    + Deref<Target = FileBase<Self>> // Deref here is fine, as files should not have any more information and should not have any more function than wrapping FileBase
    + DerefMut
{
    fn new(path: impl AsRef<Path>) -> Self;
    fn file_init_bytes() -> Option<&'static [u8]> { None }
    fn ext() -> &'static [&'static str];
}
