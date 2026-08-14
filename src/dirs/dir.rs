use std::{fs, io, ops::Div, path::{Path, PathBuf}};

use crate::{DirFile, FileType, traits::{FileTrait, FsElement}};

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when creating a [Dir].
pub enum DirCreationError {
    #[error("{0:?} is not a directory")]
    NotADir(PathBuf),
}

/// Enforces that [Path](std::path::Path) is a directory.
/// 
/// For any *available* file use [`Dir<FileType>`](crate::Dir).
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dir {
    path: PathBuf,
}

impl FsElement for Dir {
    type TryNewError = DirCreationError;

    /// Creates a new [Dir] instance from a given path.
    ///
    /// If the path already exists, it must be a directory. If it does not exist, it will be created recursively.
    fn try_new(path: impl AsRef<std::path::Path>) -> Result<Self, Self::TryNewError> {
        let dir = path.as_ref().to_path_buf();

        if dir.exists() && !dir.is_dir() {
            return Err(Self::TryNewError::NotADir(dir));
        }

        Ok(Self { path: dir })
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
impl crate::traits::AsyncFsElement for Dir {
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

impl Dir {
    /// Creates a new [Dir] instance from a given path.
    ///
    /// If the path already exists, it must be a directory. If it does not exist, it will be created recursively.
    /// 
    /// Panics at error.
    pub fn new(path: impl AsRef<Path>) -> Self {
        <Dir as FsElement>::new(path)
    }
    
    /// Creates a new [Dir] instance from a given path.
    ///
    /// If the path already exists, it must be a directory. If it does not exist, it will be created recursively.
    pub fn try_new(path: impl AsRef<Path>) -> Result<Self, <Dir as FsElement>::TryNewError> {
        <Dir as FsElement>::try_new(path)
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

    /// Uses glob pattern to find files, and converts them into [FileType](FileType)
    /// 
    /// Panics if pattern is incorrect trying to find non [FileType](FileType) files (TODO: Custom error)
    /// 
    /// For something more advanced, use [filess::glob](crate::glob) directly
    #[cfg(feature = "glob")]
    pub fn glob(&self, pattern: &str) -> Result<Vec<FileType>, glob::GlobError> {
        let mut res = vec![];
        for p in glob::glob(pattern).unwrap() {
            res.push(<FileType as FileTrait>::new(p?))
        }
        Ok(res)
    }
    
    /// Uses glob pattern with custom options to find files, and converts them into [FileType](FileType)
    /// 
    /// Panics if pattern is incorrect trying to find non [FileType](FileType) files (TODO: Custom error)
    /// 
    /// For something more advanced, use [filess::glob](crate::glob) directly
    #[cfg(feature = "glob")]
    pub fn glob_with(&self, pattern: &str, options: glob::MatchOptions) -> Result<Vec<FileType>, glob::GlobError> {
        let mut res = vec![];
        for p in glob::glob_with(pattern, options).unwrap() {
            res.push(<FileType as FileTrait>::new(p?))
        }
        Ok(res)
    }
}

impl Dir {
    /// Loads content of every file in this directory.
    pub fn load_files<F: FileTrait>(&self, files: &[F]) -> io::Result<Vec<Vec<u8>>> {
        files.iter().map(|f| f.load()).collect()
    }

    /// Saves data (one chunk per file) to every file in this directory.
    pub fn save_files<F: FileTrait>(&self, files: &[F], data: Vec<Vec<u8>>) -> io::Result<()> {
        for (f, d) in files.iter().zip(data) {
            f.save(d)?;
        }
        Ok(())
    }
}

#[cfg(feature = "async")]
impl Dir {
    /// Async version of [Dir::load_files].
    pub async fn aload_files<F: FileTrait + 'static>(&self, files: Vec<F>) -> io::Result<Vec<Vec<u8>>> {
        use crate::traits::AsyncFileTrait;
            
        let mut set = tokio::task::JoinSet::new();

        for f in files {
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

#[cfg(feature = "serde")]
impl Dir {
    /// Deserializes each byte vector using the corresponding file's format.
    pub fn self_bytes_to_models<F: crate::traits::ModelFile, T: for<'de> serde::Deserialize<'de>>(
        &self,
        files: &[F],
        data: Vec<Vec<u8>>,
    ) -> Result<Vec<T>, F::Error> {
        files
            .iter()
            .zip(data)
            .map(|(f, d)| f.self_bytes_to_model(d))
            .collect()
    }
    
    /// Loads and deserializes all files in the directory.
    pub fn load_models<F: crate::traits::ModelFile, T: for<'de> serde::Deserialize<'de>>(&self, files: &[F]) -> Result<Vec<T>, F::Error> {
        self.self_bytes_to_models(files, self.load_files(files)?)
    }
}

#[cfg(all(feature = "serde", feature = "async"))]
impl Dir {
    /// Async version of [Dir::load_models].
    pub async fn aload_models<F: crate::traits::ModelFile + 'static, T: for<'de> serde::Deserialize<'de>>(&self, files: &[F]) -> Result<Vec<T>, F::Error> {
        self.self_bytes_to_models(files, self.aload_files(files.into()).await?)
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
    type Target = Path;
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl std::ops::DerefMut for Dir {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}

impl Div<Self> for Dir {
    type Output = Self;
    fn div(self, rhs: Dir) -> Self::Output {
        Self::new(self.join(rhs))
    }
}

impl Div<&str> for Dir {
    type Output = DirFile<FileType>;
    fn div(self, rhs: &str) -> Self::Output {
        let new_path = self.join(rhs);
        if new_path.extension().is_some() {
            DirFile::File(<FileType as FileTrait>::new(self.join(rhs)))
        } else {
            DirFile::Dir(Self::new(self.join(rhs)))
        }
    }
}
