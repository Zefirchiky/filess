use std::{
    fs, io, ops::Div, path::{Path, PathBuf},
};

use crate::{DirFile, traits::FsElement};
use crate::traits::FileTrait;

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when creating a [Dir].
pub enum DirCreationError {
    #[error("{0:?} is not a directory")]
    NotADir(PathBuf),
}

/// A directory structure, simplifies work with multiple files.
/// 
/// For any available file use [`Dir<FileType>`](crate::Dir).
/// For any file or dir - [`Dir<DirFile<FileType, FileType>>`](crate::Dir)
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dir<F: FsElement> {
    path: PathBuf,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub elements: Vec<F>,
}

/// A [Dir] that can hold any file or sub-directory type.
pub type DirAny = Dir<crate::DirFileAny>;

impl<F: FsElement> FsElement for Dir<F> {
    type TryNewError = DirCreationError;
    
    /// Creates a new [Dir] instance from a given path.
    ///
    /// If the path already exists, it must be a directory. If it does not exist, it will be created recursively.
    fn try_new(dir: impl AsRef<Path>) -> Result<Self, Self::TryNewError> {
        let dir = dir.as_ref().to_path_buf();

        if dir.exists() && !dir.is_dir() {
            return Err(Self::TryNewError::NotADir(dir));
        }

        Ok(Self {
            path: dir,
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
        self.path = name.as_ref().into();
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl<F: FsElement> crate::traits::AsyncFsElement for Dir<F> {
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
}

impl<F: FsElement> Dir<F> {
    /// Creates a new [Dir] instance from a given path.
    ///
    /// If the path already exists, it must be a directory. If it does not exist, it will be created recursively.
    /// 
    /// Panics at error.
    pub fn new(path: impl AsRef<Path>) -> Self {
        <Dir<F> as FsElement>::new(path)
    }
    
    /// Creates a new [Dir] instance from a given path.
    ///
    /// If the path already exists, it must be a directory. If it does not exist, it will be created recursively.
    pub fn try_new(path: impl AsRef<Path>) -> Result<Self, <Dir<F> as FsElement>::TryNewError> {
        <Dir<F> as FsElement>::try_new(path)
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

    /// Walks through dir with [walkdir].
    ///
    /// ```ignore
    /// let dir = Dir::<File>::new("path");
    /// for entry in dir.walk() { ... }
    /// ```
    #[cfg(feature = "walk")]
    pub fn walk(&self) -> walkdir::WalkDir {
        walkdir::WalkDir::new(self)
    }

    /// Moves folder in trash
    #[cfg(feature = "trash")]
    pub fn trash_files(&self) -> Result<(), trash::Error> {
        trash::delete_all(self.iter())
    }

    /// Uses glob pattern to find files, and converts them into `F`
    /// 
    /// Panics if pattern is incorrect trying to find non `F` files (TODO: Custom error)
    /// 
    /// For something more advanced, use [filess::glob](crate::glob) directly
    #[cfg(feature = "glob")]
    pub fn glob(&self, pattern: &str) -> Vec<Result<F, glob::GlobError>> {
        glob::glob(pattern).unwrap()
            .map(|p| p.map(|f| F::new(f)))
            .collect()
    }
    
    /// Uses glob pattern with custom options to find files, and converts them into `F`
    /// 
    /// Panics if pattern is incorrect
    /// 
    /// For something more advanced, use [filess::glob](crate::glob) directly
    #[cfg(feature = "glob")]
    pub fn glob_with(&self, pattern: &str, options: glob::MatchOptions) -> Vec<Result<F, glob::GlobError>> {
        glob::glob_with(pattern, options).unwrap()
            .map(|p| p.map(|f| F::new(f)))
            .collect()
    }
}

impl<F: crate::traits::FileTrait> Dir<F> {
    /// Loads content of every file in this directory.
    pub fn load_files(&self) -> io::Result<Vec<Vec<u8>>> {
        self.elements.iter().map(|f| f.load()).collect()
    }

    /// Saves data (one chunk per file) to every file in this directory.
    pub fn save_files(&self, data: Vec<Vec<u8>>) -> io::Result<()> {
        for (f, d) in self.elements.iter().zip(data) {
            f.save(d)?;
        }
        Ok(())
    }
}

#[cfg(feature = "async")]
impl<F: crate::traits::AsyncFsElement> Dir<F> {
    /// Async version of [Dir::create_all].
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
impl<F: FileTrait + 'static> Dir<F> {
    /// Async version of [Dir::load_files].
    pub async fn aload_files(&self) -> io::Result<Vec<Vec<u8>>> {
        use crate::traits::AsyncFileTrait;
            
        let mut set = tokio::task::JoinSet::new();

        for f in self {
            let f = f.clone();
            set.spawn(async move {
                f.aload().await
            });
        }

        let mut results = Vec::new();
        while let Some(res) = set.join_next().await {
            // res? checks if the task panicked
            // res?? checks if aload() returned an Err
            results.push(res??);
        }

        Ok(results)
    }
}

#[cfg(feature = "_any_serde_model")]
impl<F: crate::traits::ModelFile + 'static> Dir<F> {
    /// Deserializes each byte vector using the corresponding file's format.
    pub fn self_bytes_to_models<T: for<'de> serde::Deserialize<'de>>(
        &self,
        data: Vec<Vec<u8>>,
    ) -> Result<Vec<T>, F::Error> {
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

    /// Async version of [Dir::load_models].
    #[cfg(feature = "async")]
    pub async fn aload_models<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<Vec<T>, F::Error> {
        self.self_bytes_to_models(self.aload_files().await?)
    }
}

impl<F: FsElement> AsRef<std::path::Path> for Dir<F> {
    fn as_ref(&self) -> &std::path::Path {
        &self.path
    }
}

impl<F: FsElement> From<&std::path::Path> for Dir<F> {
    fn from(path: &std::path::Path) -> Self {
        Self::new(path)
    }
}

impl<F: FsElement> From<std::path::PathBuf> for Dir<F> {
    fn from(path: std::path::PathBuf) -> Self {
        Self::new(path)
    }
}

impl<F: FsElement> From<&str> for Dir<F> {
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}

impl<F: FsElement> From<String> for Dir<F> {
    fn from(path: String) -> Self {
        Self::new(path)
    }
}

impl<F: FsElement> std::ops::Deref for Dir<F> {
    type Target = Path;
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl<F: FsElement> std::ops::DerefMut for Dir<F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}

impl<F: FsElement> Div<Self> for Dir<F> {
    type Output = Self;
    fn div(self, rhs: Dir<F>) -> Self::Output {
        Self::new(self.join(rhs))
    }
}

impl<F: FileTrait> Div<&str> for Dir<F> {
    type Output = DirFile<F, F>;
    fn div(self, rhs: &str) -> Self::Output {
        let new_path = self.join(rhs);
        if new_path.extension().is_some() {
            DirFile::File(<F as FsElement>::new(self.join(rhs)))
        } else {
            DirFile::Dir(Self::new(self.join(rhs)))
        }
    }
}

impl<F: FileTrait> IntoIterator for Dir<F> {
    type Item = F;
    type IntoIter = <Vec<F> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, F: FileTrait> IntoIterator for &'a Dir<F> {
    type Item = &'a F;
    type IntoIter = <&'a Vec<F> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a, F: FileTrait> IntoIterator for &'a mut Dir<F> {
    type Item = &'a mut F;
    type IntoIter = <&'a mut Vec<F> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}
