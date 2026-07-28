use crate::{Dir, FileType, traits::FsElement};

/// Represents either file `F1` or a dir containing `F2`
#[derive(Debug)]
pub enum DirFile<F1: FsElement, F2: FsElement> {
    Dir(Dir<F1>),
    File(F2),
}

pub type DirFileAny =
    DirFile<DirFile<DirFile<DirFile<FileType, FileType>, FileType>, FileType>, FileType>;   // FIXME: WHAT THE FUCK IS THIS 😭
