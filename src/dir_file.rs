use crate::{Dir, FileType, traits::FsElement};

/// Represents either file `F1` or a dir containing `F2`
#[derive(Debug)]
pub enum DirFile<F1: FsElement, F2: FsElement> {
    Dir(Dir<F1>),
    File(F2),
}

/// Recursive type alias allowing [Dir](crate::Dir) to nest files and sub-dirs arbitrarily.
// FIXME: This type is overly nested and hard to use.
pub type DirFileAny =
    DirFile<DirFile<DirFile<DirFile<FileType, FileType>, FileType>, FileType>, FileType>;
