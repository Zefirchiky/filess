use std::path::PathBuf;

use crate::{Dir, FileType, traits::{FileTrait, FsElement}};

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when creating a [Dir].
pub enum DirFileCreationError<F: FileTrait> {
    #[error("Dir creation error: {0:?}")]
    Dir(#[from] <crate::Dir as FsElement>::TryNewError),
    #[error("File creation error: {0:?}")]
    File(F::TryNewError),
}

impl<F: FileTrait> From<crate::file_bases::file_base::FileCreationError<F>> for DirFileCreationError<F> {
    fn from(err: crate::file_bases::file_base::FileCreationError<F>) -> Self {
        Self::File(err)
    }
}

/// Represents either [Dir](crate::Dir) or `F`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = "F: serde::Serialize + serde::de::DeserializeOwned"))]
pub enum DirFile<F: FileTrait> {
    Dir(Dir),
    File(F),
}

/// Recursive type alias, represent [DirFile]  any *available* file
pub type DirFileAny = DirFile<FileType>;

impl<F: FileTrait> AsRef<std::path::Path> for DirFile<F> {
    fn as_ref(&self) -> &std::path::Path {
        match self {
            DirFile::Dir(d) => d.as_ref(),
            DirFile::File(f) => f.as_ref(),
        }
    }
}

impl<F: FileTrait> From<&std::path::Path> for DirFile<F> {
    fn from(path: &std::path::Path) -> Self {
        Self::new(path)
    }
}

impl<F: FileTrait> From<PathBuf> for DirFile<F> {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl<F: FileTrait> From<String> for DirFile<F> {
    fn from(path: String) -> Self {
        Self::new(path)
    }
}

impl<F: FileTrait> From<&'static str> for DirFile<F> {
    fn from(path: &'static str) -> Self {
        Self::new(path)
    }
}

impl<F: FileTrait> Default for DirFile<F> {
    fn default() -> Self {
        Self::File(F::default())
    }
}

impl<F: FileTrait> FsElement for DirFile<F> {
    type TryNewError = DirFileCreationError<F>;

    fn try_new(path: impl AsRef<std::path::Path>) -> Result<Self, Self::TryNewError> {
        if path.as_ref().extension().is_some() {
            Ok(Self::File(<F as FileTrait>::try_new(path)?))
        } else {
            Ok(Self::Dir(Dir::try_new(path)?))
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
impl<F: FileTrait> crate::traits::AsyncFsElement for DirFile<F> {
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
