use std::{
    ffi::OsStr, fmt::Debug, fs::{self, create_dir_all}, marker::PhantomData, ops::{Deref, DerefMut}, path::{Path, PathBuf}
};

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

#[cfg(feature = "open")]
impl<F: FileTrait> crate::primitives::OpenTrait for FileBase<F> {}

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

pub trait FileTrait:
    Debug + Clone + Default + From<PathBuf> + From<&'static str> + AsRef<Path> + AsMut<Path>
{
    fn change_path(&mut self, path: PathBuf);
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
    
    /// Corresponds to `fs::copy`.
    /// 
    /// Copies file to the new path, you will still have this instance.
    /// New file instance will be returned.
    fn copy(&self, path: &impl AsRef<Path>) -> std::io::Result<Self> {
        fs::copy(self, path)?;
        Ok(Self::new(path))
        
    }
    
    /// Corresponds to `fs::rename`.
    /// 
    /// Moves the file to the new path, you will still have this instance.
    /// New file instance will be returned.
    fn rename(&self, path: &impl AsRef<Path>) -> std::io::Result<Self> {
        fs::rename(self, path)?;
        Ok(Self::new(path))
    }
    
    /// Changes this instance's name.
    /// 
    /// Different from `fs::rename` in that only the filename will be changed, it will stay in the same directory.
    /// 
    /// Changes this instance.
    fn rename_file(&mut self, name: &impl AsRef<OsStr>) -> std::io::Result<()> {
        let parent = self.as_ref().parent();
        fs::rename(&self, &parent.expect("It's a file, can't be root").join(PathBuf::from(name.as_ref())))?;
        self.change_path(self.as_ref().with_file_name(name));
        Ok(())
    }

    /// Opens file in default program using `open::that_detached()`
    ///
    /// For other methods use `open` crate directly with `&file.as_ref()`
    #[cfg(feature = "open")]
    fn open(&self) -> std::io::Result<()> {
        open::that_detached(&self.as_ref())
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
