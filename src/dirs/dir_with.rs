use std::{
    fs, io, ops::Div, path::{Path},
};

use crate::{Dir, DirWithFile, traits::FsElement};
use crate::traits::FileTrait;

/// Simplifies work with multiple files and enforced that [Path] is a directory.
/// 
/// For any *available* file use [DirAny].
///
/// To simply enforce that [Path] is a directory, use [crate::Dir]
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirWith<F: FsElement> {
    pub dir: Dir,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub elements: Vec<F>,
}

/// A [Dir] that can hold any file or sub-directory type.
pub type DirAny = DirWith<crate::DirWithFileAny>;

impl<F: FsElement> FsElement for DirWith<F> {
    type TryNewError = <Dir as FsElement>::TryNewError;
    
    /// Creates a new [DirWith] instance from a given path.
    ///
    /// If the path already exists, it must be a directory. If it does not exist, it will be created recursively.
    fn try_new(path: impl AsRef<Path>) -> Result<Self, Self::TryNewError> {
        Ok(Self {
            dir: Dir::try_new(path)?,
            elements: vec![],
        })
    }

    /// Recursively creates a dir in file system.
    fn create(&self) -> std::io::Result<()> {
        fs::create_dir_all(self)
    }

    /// Removes a dir from file system.
    /// 
    /// !!! INCLUDING THE CONTENT INSIDE !!!
    fn remove(&self) -> std::io::Result<()> {
        fs::remove_dir_all(self)
    }

    fn rename_file(&mut self, name: impl AsRef<Path>) {
        self.dir = name.as_ref().into();
    }

    /// Recursively copies this directory and its elements to the new path.
    fn copy(&self, dst: impl AsRef<Path>) -> std::io::Result<Self> {
        // Copy the directory contents via Dir::copy
        self.dir.copy(&dst)?;
        Ok(Self::new(dst))
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl<F: FsElement> crate::traits::AsyncFsElement for DirWith<F> {
    /// Recursively creates dir in file system
    async fn acreate(&self) -> std::io::Result<()> {
        tokio::fs::create_dir_all(self).await
    }
    
    /// Removes a dir from file system.
    /// 
    /// !!! INCLUDING THE CONTENT INSIDE !!!
    async fn aremove(&self) -> std::io::Result<()> {
        tokio::fs::remove_dir_all(self).await
    }

    async fn acopy(&self, dst: impl AsRef<Path> + Sync + Send) -> std::io::Result<Self> {
        self.dir.acopy(&dst).await?;
        Ok(Self::new(dst))
    }
}

impl<F: FsElement> DirWith<F> {
    /// Creates a new [DirWith] instance from a given path.
    ///
    /// If the path already exists, it must be a directory. If it does not exist, it will be created recursively.
    /// 
    /// Panics at error.
    pub fn new(path: impl AsRef<Path>) -> Self {
        <DirWith<F> as FsElement>::new(path)
    }
       
    /// Creates a new [DirWith] instance from a given path.
    ///
    /// If the path already exists, it must be a directory. If it does not exist, it will be created recursively.
    pub fn try_new(path: impl AsRef<Path>) -> Result<Self, <DirWith<F> as FsElement>::TryNewError> {
        <DirWith<F> as FsElement>::try_new(path)
    }

    /// Creates this directory and recursively creates all child files and directories.
    ///
    /// For async, use [acreate_all](Self::acreate_all).
    pub fn create_all(&self) -> io::Result<()> {
        self.create()?;
        for file in &self.elements {
            file.create()?;
        }

        Ok(())
    }

    /// Push an element to this directory.
    ///
    /// ```ignore
    /// let mut dir = Dir::<Json>::new("path");
    /// dir.push(Json::new("file.json"));
    /// ```
    pub fn push(&mut self, file: F) {
        self.elements.push(file)
    }

    /// Moves folder in trash
    #[cfg(feature = "trash")]
    pub fn trash_files(&self) -> Result<(), trash::Error> {
        trash::delete(self)
    }

    /// Uses glob pattern to find files, and converts them into `F`
    /// 
    /// Panics if pattern is incorrect trying to find non `F` files (TODO: Custom error)
    /// 
    /// For something more advanced, use [filess::glob](crate::glob) directly
    #[cfg(feature = "glob")]
    pub fn glob(&self, pattern: &str) -> Result<Vec<F>, glob::GlobError> {
        let mut res = vec![];
        for p in glob::glob(&self.join(pattern).to_string_lossy()).unwrap() {
            res.push(F::new(p?))
        }
        Ok(res)
    }
    
    /// Uses glob pattern with custom options to find files, and converts them into `F`
    /// 
    /// Panics if pattern is incorrect trying to find non `F` files (TODO: Custom error)
    /// 
    /// For something more advanced, use [filess::glob](crate::glob) directly
    #[cfg(feature = "glob")]
    pub fn glob_with(&self, pattern: &str, options: glob::MatchOptions) -> Result<Vec<F>, glob::GlobError> {
        let mut res = vec![];
        for p in glob::glob_with(&self.join(pattern).to_string_lossy(), options).unwrap() {
            res.push(F::new(p?))
        }
        Ok(res)
    }
}

#[cfg(feature = "dirs")]
impl<F: crate::traits::FileTrait> DirWith<F> {
    /// Creates a [DirWith] from the user's home directory.
    pub fn home_dir() -> Option<Self> {
        dirs::home_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's config directory.
    pub fn config_dir() -> Option<Self> {
        dirs::config_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's cache directory.
    pub fn cache_dir() -> Option<Self> {
        dirs::cache_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's data directory.
    pub fn data_dir() -> Option<Self> {
        dirs::data_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's desktop directory.
    pub fn desktop_dir() -> Option<Self> {
        dirs::desktop_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's documents directory.
    pub fn document_dir() -> Option<DirWith<crate::TextTypes>> {
        dirs::document_dir().map(DirWith::new)
    }

    /// Creates a [DirWith] from the user's downloads directory.
    pub fn download_dir() -> Option<Self> {
        dirs::download_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's pictures directory.
    #[cfg(feature = "image")]
    pub fn picture_dir() -> Option<DirWith<crate::ImageTypes>> {
        dirs::picture_dir().map(DirWith::new)
    }
    /// Creates a [DirWith] from the user's pictures directory.
    #[cfg(not(feature = "image"))]
    pub fn picture_dir() -> Option<Self> {
        dirs::picture_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's music directory.
    #[cfg(feature = "audio")]
    pub fn audio_dir() -> Option<DirWith<crate::AudioTypes>> {
        dirs::audio_dir().map(DirWith::new)
    }
    /// Creates a [DirWith] from the user's music directory.
    #[cfg(not(feature = "audio"))]
    pub fn audio_dir() -> Option<Self> {
        dirs::audio_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's video directory.
    pub fn video_dir() -> Option<Self> {    // TODO: VideoTypes
        dirs::video_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's runtime directory.
    pub fn runtime_dir() -> Option<Self> {
        dirs::runtime_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's fonts directory.
    pub fn font_dir() -> Option<Self> {
        dirs::font_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's executable directory.
    pub fn executable_dir() -> Option<Self> {
        dirs::executable_dir().map(Self::new)
    }

    /// Creates a [DirWith] from the user's preference directory.
    pub fn preference_dir() -> Option<Self> {
        dirs::preference_dir().map(Self::new)
    }
}

impl<F: crate::traits::FileTrait> DirWith<F> {
    /// Loads content of every file in this directory.
    pub fn load_files(&self) -> io::Result<Vec<Vec<u8>>> {
        self.elements.iter().map(|f| f.load()).collect()
    }

    /// Saves data (one chunk per file) to every file in this directory.
    pub fn save_files(&self, data: Vec<Vec<u8>>) -> io::Result<()> {
        if self.elements.len() != data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "elements and data must have the same length",
            ));
        }
        for (f, d) in self.elements.iter().zip(data) {
            f.save(d)?;
        }
        Ok(())
    }
}

#[cfg(feature = "async")]
impl<F: crate::traits::AsyncFsElement> DirWith<F> {
    /// Async version of [DirWith::create_all].
    pub async fn acreate_all(&self) -> io::Result<()> {
        use crate::traits::AsyncFsElement;

        self.acreate().await?;
        for el in &self.elements {
            el.acreate().await?;
        }
        Ok(())
    }
}

#[cfg(feature = "async")]
impl<F: FileTrait + 'static> DirWith<F> {
    /// Async version of [Dir::load_files].
    pub async fn aload_files(&self) -> io::Result<Vec<Vec<u8>>> {
        use crate::traits::AsyncFileTrait;
            
        let mut set = tokio::task::JoinSet::new();

        for (i, f) in self.into_iter().enumerate() {
            let f = f.clone();
            set.spawn(async move {
                (i, f.aload().await)
            });
        }

        let mut results = Vec::new();
        while let Some(res) = set.join_next().await {
            // res? checks if the task panicked
            // r? checks if aload() returned an Err
            let (i, r) = res?;
            results.push((i, r?));
        }

        // JoinSet completes in nondeterministic order — restore input order
        results.sort_by_key(|(i, _)| *i);
        Ok(results.into_iter().map(|(_, r)| r).collect())
    }
}

#[cfg(feature = "serde")]
impl<F: crate::traits::ModelFile > DirWith<F> {
    /// Deserializes each byte vector using the corresponding file's format.
    pub fn self_bytes_to_models<T: for<'de> serde::Deserialize<'de>>(
        &self,
        data: Vec<Vec<u8>>,
    ) -> Result<Vec<T>, F::Error> {
        if self.elements.len() != data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "elements and data must have the same length",
            ).into());
        }
        self.elements
            .iter()
            .zip(data)
            .map(|(f, d)| f.self_bytes_to_model(d))
            .collect()
    }
    
    /// Loads and deserializes all files in the directory.
    pub fn load_models<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<Vec<T>, F::Error> {
        self.self_bytes_to_models(self.load_files()?)
    }
}

#[cfg(all(feature = "serde", feature = "async"))]
impl<F: crate::traits::ModelFile + 'static> DirWith<F> {
    /// Async version of [Dir::load_models].
    pub async fn aload_models<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<Vec<T>, F::Error> {
        self.self_bytes_to_models(self.aload_files().await?)
    }
}

impl<F: FsElement> AsRef<std::path::Path> for DirWith<F> {
    fn as_ref(&self) -> &std::path::Path {
        &self.dir.as_ref()
    }
}

impl<F: FsElement> From<&std::path::Path> for DirWith<F> {
    fn from(path: &std::path::Path) -> Self {
        Self::new(path)
    }
}

impl<F: FsElement> From<std::path::PathBuf> for DirWith<F> {
    fn from(path: std::path::PathBuf) -> Self {
        Self::new(path)
    }
}

impl<F: FsElement> From<&str> for DirWith<F> {
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}

impl<F: FsElement> From<String> for DirWith<F> {
    fn from(path: String) -> Self {
        Self::new(path)
    }
}

impl<F: FsElement> std::ops::Deref for DirWith<F> {
    type Target = Dir;
    fn deref(&self) -> &Self::Target {
        &self.dir
    }
}

impl<F: FsElement> std::ops::DerefMut for DirWith<F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dir
    }
}

impl<F: FsElement> Div<Self> for DirWith<F> {
    type Output = Self;
    fn div(self, rhs: DirWith<F>) -> Self::Output {
        Self::new(self.join(rhs))
    }
}

impl<F: FileTrait> Div<&str> for DirWith<F> {
    type Output = DirWithFile<F, F>;
    fn div(self, rhs: &str) -> Self::Output {
        let new_path = self.join(rhs);
        if new_path.extension().is_some() {
            DirWithFile::File(<F as FsElement>::new(self.join(rhs)))
        } else {
            DirWithFile::Dir(Self::new(self.join(rhs)))
        }
    }
}

impl<F: FileTrait> IntoIterator for DirWith<F> {
    type Item = F;
    type IntoIter = <Vec<F> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, F: FileTrait> IntoIterator for &'a DirWith<F> {
    type Item = &'a F;
    type IntoIter = <&'a Vec<F> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a, F: FileTrait> IntoIterator for &'a mut DirWith<F> {
    type Item = &'a mut F;
    type IntoIter = <&'a mut Vec<F> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}
