use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::traits::FsElement;

/// Makes an `H` be deleted after drop, together with it's empty parent dirs
#[derive(Debug, Clone)]
pub struct Temporary<H: FsElement> {
    inner: H,
}

impl<H: FsElement> Temporary<H> {
    /// Creates new temporary files, that will be deleted after drop
    pub fn new(handler: H) -> Self {
        Self { inner: handler }
    }
}

impl<H: FsElement> AsRef<Path> for Temporary<H> {
    fn as_ref(&self) -> &Path {
        &self.inner.as_ref()
    }
}

impl<H: FsElement> From<H> for Temporary<H> {
    fn from(path: H) -> Self {
        Self::new(path)
    }
}

impl<H: FsElement> Deref for Temporary<H> {
    type Target = H;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<H: FsElement> DerefMut for Temporary<H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: FsElement> Drop for Temporary<T> {
    fn drop(&mut self) {
        let _ = self.inner.remove();    // FIXME: Ignoring the error might not be the best choice
    }
}
