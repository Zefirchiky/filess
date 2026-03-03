use std::fs;

use derive_more::{AsRef, Deref, DerefMut, From};

use crate::FileTrait;

#[derive(Debug, Clone, From, AsRef, Deref, DerefMut)]
// #[from(forward)]
#[as_ref(forward)]
pub struct Temporary<H: FileTrait> {
    inner: H,
}

impl<H: FileTrait> Temporary<H> {
    pub fn new(handler: H) -> Self {
        Self { inner: handler }
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
