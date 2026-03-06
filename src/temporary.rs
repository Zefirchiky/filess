use std::{
    fs,
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::primitives::FileTrait;

/// Makes an `H` be deleted after drop, together with it's empty parent dirs
#[derive(Debug, Clone)]
pub struct Temporary<H: FileTrait> {
    inner: H,
}

impl<H: FileTrait> Temporary<H> {
    /// Creates new temporary files, that will be deleted after drop
    pub fn new(handler: H) -> Self {
        Self { inner: handler }
    }
}

impl<H: FileTrait> AsRef<Path> for Temporary<H> {
    fn as_ref(&self) -> &Path {
        &self.inner.as_ref()
    }
}

impl<H: FileTrait> From<H> for Temporary<H> {
    fn from(path: H) -> Self {
        Self::new(path)
    }
}

impl<H: FileTrait> Deref for Temporary<H> {
    type Target = H;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<H: FileTrait> DerefMut for Temporary<H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: FileTrait> Drop for Temporary<T> {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.inner);
        for dir in self.as_ref().parent().into_iter().rev() {
            if fs::remove_dir(dir).is_err() {
                break;
            }
        }
    }
}
