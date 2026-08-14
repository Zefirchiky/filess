use std::path::PathBuf;

use crate::{DirWith, FileType, traits::{FileTrait, FsElement}};

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when creating a [Dir].
pub enum DirWithFileCreationError<F1: FsElement, F2: FileTrait> {
    #[error("Dir creation error: {0:?}")]
    Dir(#[from] <DirWith<F1> as FsElement>::TryNewError),
    #[error("File creation error: {0:?}")]
    File(F2::TryNewError),
}

impl<F1: FsElement, F2: FileTrait> From<crate::file_bases::file_base::FileCreationError<F2>> for DirWithFileCreationError<F1, F2> {
    fn from(err: crate::file_bases::file_base::FileCreationError<F2>) -> Self {
        Self::File(err)
    }
}

/// Represents either dir containing `F1` or a file `F2`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = "F1: serde::Serialize + serde::de::DeserializeOwned, F2: serde::Serialize + serde::de::DeserializeOwned"))]
pub enum DirWithFile<F1: FsElement, F2: FileTrait> {
    Dir(DirWith<F1>),
    File(F2),
}

/// Recursive type alias allowing [Dir](crate::Dir) to nest files and sub-dirs arbitrarily.
pub type DirWithFileAny =
    DirWithFile<DirWithFile<DirWithFile<DirWithFile<FileType, FileType>, FileType>, FileType>, FileType>;

impl<F1: FsElement, F2: FileTrait> AsRef<std::path::Path> for DirWithFile<F1, F2> {
    fn as_ref(&self) -> &std::path::Path {
        match self {
            Self::Dir(d) => d.as_ref(),
            Self::File(f) => f.as_ref(),
        }
    }
}

impl<F1: FsElement, F2: FileTrait> From<&std::path::Path> for DirWithFile<F1, F2> {
    fn from(path: &std::path::Path) -> Self {
        Self::new(path)
    }
}

impl<F1: FsElement, F2: FileTrait> From<PathBuf> for DirWithFile<F1, F2> {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl<F1: FsElement, F2: FileTrait> From<String> for DirWithFile<F1, F2> {
    fn from(path: String) -> Self {
        Self::new(path)
    }
}

impl<F1: FsElement, F2: FileTrait> From<&'static str> for DirWithFile<F1, F2> {
    fn from(path: &'static str) -> Self {
        Self::new(path)
    }
}

impl<F1: FsElement, F2: FileTrait> Default for DirWithFile<F1, F2> {
    fn default() -> Self {
        Self::File(F2::default())
    }
}

impl<F1: FsElement, F2: FileTrait> FsElement for DirWithFile<F1, F2> {
    type TryNewError = DirWithFileCreationError<F1, F2>;

    fn try_new(path: impl AsRef<std::path::Path>) -> Result<Self, Self::TryNewError> {
        if path.as_ref().extension().is_some() {
            Ok(Self::File(<F2 as FileTrait>::try_new(path)?))
        } else {
            Ok(Self::Dir(DirWith::try_new(path)?))
        }
    }

    fn create(&self) -> std::io::Result<()> {
        match self {
            Self::Dir(d) => d.create(),
            Self::File(f) => f.create(),
        }
    }

    fn remove(&self) -> std::io::Result<()> {
        match self {
            Self::Dir(d) => d.remove(),
            Self::File(f) => f.remove(),
        }
    }

    fn rename_file(&mut self, name: impl AsRef<std::path::Path>) {
        match self {
            Self::Dir(d) => d.rename_file(name),
            Self::File(f) => f.rename_file(name),
        }
    }

    fn copy(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        match self {
            Self::Dir(d) => Ok(Self::Dir(d.copy(path)?)),
            Self::File(f) => Ok(Self::File(f.copy(path)?)),
        }
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl<F1: FsElement, F2: FileTrait> crate::traits::AsyncFsElement for DirWithFile<F1, F2> {
    async fn acreate(&self) -> std::io::Result<()> {
        match self {
            Self::Dir(d) => d.acreate().await,
            Self::File(f) => f.acreate().await,
        }
    }
    async fn aremove(&self) -> std::io::Result<()> {
        match self {
            Self::Dir(d) => d.aremove().await,
            Self::File(f) => f.aremove().await,
        }
    }
    async fn acopy(&self, path: impl AsRef<std::path::Path> + Sync + Send) -> std::io::Result<Self> {
        match self {
            Self::Dir(d) => Ok(Self::Dir(d.acopy(path).await?)),
            Self::File(f) => Ok(Self::File(f.acopy(path).await?)),
        }
    }
    async fn arename(&self, path: impl AsRef<std::path::Path> + Sync + Send) -> std::io::Result<Self> {
        match self {
            Self::Dir(d) => Ok(Self::Dir(d.arename(path).await?)),
            Self::File(f) => Ok(Self::File(f.arename(path).await?)),
        }
    }
}
