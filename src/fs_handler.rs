use crate::{Dir, FileTrait};

pub enum FsHandler<F: FileTrait> {
    Dir(Dir<F>),
    File(F),
}
