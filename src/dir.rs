use std::{
    fs, io, ops::Div, path::{Path, PathBuf},
};

use crate::{DirFile, primitives::FsElement};
use crate::primitives::FileTrait;

#[derive(Debug, thiserror::Error)]
pub enum DirCreationError {
    #[error("{0:?} is not a directory")]
    NotADir(PathBuf),
}

/// A directory structure, simplifies work with multiple files.
/// 
/// For any available file use `Dir<FileType>`.
/// For any file or dir - `Dir<DirFile<FileType, FileType>>`
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dir<F: FsElement> {
    path: PathBuf,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub elements: Vec<F>,
}

pub type DirAny = Dir<crate::DirFileAny>;

impl<F: FsElement> FsElement for Dir<F> {
    type TryNewError = DirCreationError;
    
    /// Creates a new `Dir` instance from a given path.
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
impl<F: FsElement> crate::primitives::AsyncFsElement for Dir<F> {
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

impl<F: FileTrait> Dir<F> {
    /// Will create a this directory and recursively create all child files and directories.
    ///
    /// It is recommended to use async version of this method
    pub fn create_all(&self) -> io::Result<()> {
        self.create()?;
        for file in &self.elements {
            file.create()?;
        }

        Ok(())
    }

    #[cfg(feature = "async")]
    pub async fn acreate_all(&self) -> io::Result<()> {
        self.acreate().await?;
        Ok(())
    }

    #[cfg(feature = "async")]
    pub async fn acreate(&self) -> io::Result<()> {
        tokio::fs::create_dir_all(&self).await
    }

    /// Adds a file to this directory. Path should be relative to the folder
    pub fn add(&mut self, file: F) {
        self.elements.push(file)
    }

    pub fn load_files(&self) -> io::Result<Vec<Vec<u8>>> {
        self.elements.iter().map(|f| f.load()).collect()
    }

    #[cfg(feature = "walk")]
    pub fn walk(&self) -> walkdir::WalkDir {
        walkdir::WalkDir::new(&self)
    }

    /// Opens directory in default program using `open::that_detached()`
    ///
    /// For other methods use `open` crate directly with `&file.as_ref()`
    #[cfg(feature = "open")]
    pub fn open(&self) -> std::io::Result<()> {
        open::that_detached(&self.as_ref())
    }

    /// Moves folder in trash
    #[cfg(feature = "trash")]
    pub fn trash(&self) -> Result<(), trash::Error> {
        trash::delete_all(self)
    }
}

impl<F: FileTrait + 'static> Dir<F> {
    #[cfg(feature = "async")]
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

#[cfg(all(feature = "serde", any(feature = "serde_json", feature = "serde_toml")))]
impl<F: crate::traits::ModelFile + 'static> Dir<F> {
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

    pub fn load_models<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<Vec<T>, F::Error> {
        Ok(self.self_bytes_to_models(self.load_files()?)?)
    }

    pub async fn aload_models<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<Vec<T>, F::Error> {
        Ok(self.self_bytes_to_models(self.aload_files().await?)?)
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

impl<F: FsElement> Div<&str> for Dir<F> {
    type Output = DirFile<F, F>;
    fn div(self, rhs: &str) -> Self::Output {
        let new_path = self.join(rhs);
        if let Some(_) = new_path.extension() {
            DirFile::File(F::new(&self.join(rhs)))
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
