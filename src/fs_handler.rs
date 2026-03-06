use crate::{Dir, primitives::FileTrait};

pub enum FsHandler<F: FileTrait> {
    Dir(Dir<F>),
    File(F),
}
