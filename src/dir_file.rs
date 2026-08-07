use crate::{Dir, FileType, traits::{FileTrait, FsElement}};

/// Represents either file `F1` or a dir containing `F2`
#[derive(Debug)]
pub enum DirFile<F1: FsElement, F2: FileTrait> {
    Dir(Dir<F1>),
    File(F2),
}

impl<F1: FsElement, F2: FileTrait> Clone for DirFile<F1, F2> {
    fn clone(&self) -> Self {
        match self {
            Self::Dir(d) => Self::Dir(d.clone()),
            Self::File(f) => Self::File(f.clone()),
        }
    }
}

impl<F1: FsElement, F2: FileTrait> AsRef<std::path::Path> for DirFile<F1, F2> {
    fn as_ref(&self) -> &std::path::Path {
        match self {
            Self::Dir(d) => d.as_ref(),
            Self::File(f) => f.as_ref(),
        }
    }
}

impl<F1: FsElement, F2: FileTrait> From<std::path::PathBuf> for DirFile<F1, F2> {
    fn from(path: std::path::PathBuf) -> Self {
        Self::File(<F2 as FsElement>::new(path))
    }
}

impl<F1: FsElement, F2: FileTrait> From<&'static str> for DirFile<F1, F2> {
    fn from(path: &'static str) -> Self {
        Self::File(<F2 as FsElement>::new(path))
    }
}

impl<F1: FsElement, F2: FileTrait> FsElement for DirFile<F1, F2> {
    type TryNewError = F2::TryNewError;

    fn try_new(path: impl AsRef<std::path::Path>) -> Result<Self, Self::TryNewError> {
        Ok(Self::File(<F2 as FsElement>::try_new(path)?))
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
impl<F1: FsElement, F2: FileTrait> crate::traits::AsyncFsElement for DirFile<F1, F2> {
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

/// Recursive type alias allowing [Dir](crate::Dir) to nest files and sub-dirs arbitrarily.
pub type DirFileAny =
    DirFile<DirFile<DirFile<DirFile<FileType, FileType>, FileType>, FileType>, FileType>;
