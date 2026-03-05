use std::{
    io,
    ops::Div,
    path::{Path, PathBuf},
};

use crate::FileTrait;
pub use crate::FsHandler;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dir<F: FileTrait> {
    path: PathBuf,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub files: Vec<F>,
}

impl<F: FileTrait> Dir<F> {
    /// Creates a new `Dir` instance from a given path.
    ///
    /// If the path already exists, it must be a directory. If it does not exist, it will be created recursively.
    ///
    /// # Panics
    ///
    /// If the path cannot be created, a panic will occur with the error message.
    pub fn new(dir: impl AsRef<Path>) -> Self {
        let dir = dir.as_ref().to_path_buf();

        if dir.exists() {
            assert!(!dir.is_dir(), "{dir:?} is not a directory")
        }

        Self {
            path: dir,
            files: vec![],
        }
    }

    /// Will create a this directory and recursively create all child files and directories.
    ///
    /// It is recommended to use async version of this method
    pub fn create_all(&self) -> io::Result<()> {
        self.create()?;
        for file in &self.files {
            file.create()?;
        }

        Ok(())
    }

    #[cfg(feature = "async")]
    pub async fn acreate_all(&self) -> io::Result<()> {
        self.acreate().await?;

        Ok(())
    }

    pub fn create(&self) -> io::Result<()> {
        std::fs::create_dir_all(&self)
    }

    #[cfg(feature = "async")]
    pub async fn acreate(&self) -> io::Result<()> {
        tokio::fs::create_dir_all(&self).await
    }

    /// Adds a file to this directory. Path should be relative to the folder
    pub fn add(&mut self, file: F) {
        self.files.push(file)
    }

    pub fn load_files(&self) -> io::Result<Vec<Vec<u8>>> {
        self.files.iter().map(|f| f.load()).collect()
    }

    // ! AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
    // #[cfg(feature = "async")]
    // pub async fn aload_files(&self) -> io::Result<Vec<Vec<u8>>> {
    //     use crate::FileTraitAsync;

    //     let mut set = tokio::task::JoinSet::new();

    //     for f in self {
    //         // Spawn each load as a background task
    //         set.spawn(async move { f.aload().await });
    //     }

    //     let mut results = Vec::new();
    //     while let Some(res) = set.join_next().await {
    //         // res? checks if the task panicked
    //         // res?? checks if aload() returned an Err
    //         results.push(res??);
    //     }

    //     Ok(results)
    // }
}

#[cfg(all(feature = "serde", any(feature = "serde_json", feature = "serde_toml")))]
impl<F: crate::ModelFile> Dir<F> {
    pub fn self_bytes_to_models<T: for<'de> serde::Deserialize<'de>>(
        &self,
        data: Vec<Vec<u8>>,
    ) -> Result<Vec<T>, F::Error> {
        self.files.iter()
            .zip(data)
            .map(|(f, d)| f.self_bytes_to_model(d))
            .collect()
    }

    pub fn load_models<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<Vec<T>, F::Error> {
        Ok(self.self_bytes_to_models(self.load_files()?)?)
    }

    // pub async fn aload_models<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<Vec<T>, F::Error> {
    //     Ok(self.self_bytes_to_models(self.aload_files().await?)?)
    // }
}

impl<F: FileTrait> AsRef<std::path::Path> for Dir<F> {
    fn as_ref(&self) -> &std::path::Path {
        &self.path
    }
}

impl<F: FileTrait> From<&std::path::Path> for Dir<F> {
    fn from(path: &std::path::Path) -> Self {
        Self::new(path)
    }
}

impl<F: FileTrait> From<std::path::PathBuf> for Dir<F> {
    fn from(path: std::path::PathBuf) -> Self {
        Self::new(path)
    }
}

impl<F: FileTrait> From<&str> for Dir<F> {
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}

impl<F: FileTrait> From<String> for Dir<F> {
    fn from(path: String) -> Self {
        Self::new(path)
    }
}

impl<F: FileTrait> std::ops::Deref for Dir<F> {
    type Target = Path;
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl<F: FileTrait> std::ops::DerefMut for Dir<F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}

impl<F: FileTrait> Div<Self> for Dir<F> {
    type Output = Self;
    fn div(self, rhs: Dir<F>) -> Self::Output {
        Self::new(self.join(rhs))
    }
}

impl<F: FileTrait> Div<&str> for Dir<F> {
    type Output = FsHandler<F>;
    fn div(self, rhs: &str) -> Self::Output {
        let new_path = self.join(rhs);
        if let Some(_) = new_path.extension() {
            FsHandler::File(F::new(&self.join(rhs)))
        } else {
            FsHandler::Dir(Self::new(self.join(rhs)))
        }
    }
}

impl<F: FileTrait> IntoIterator for Dir<F> {
    type Item = F;
    type IntoIter = <Vec<F> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}

impl<'a, F: FileTrait> IntoIterator for &'a Dir<F> {
    type Item = &'a F;
    type IntoIter = <&'a Vec<F> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.files.iter()
    }
}

impl<'a, F: FileTrait> IntoIterator for &'a mut Dir<F> {
    type Item = &'a mut F;
    type IntoIter = <&'a mut Vec<F> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.files.iter_mut()
    }
}
