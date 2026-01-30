use crate::{Dir, FileTrait};

pub enum FsHandler<H: FileTrait> {
    Dir(Dir),
    File(H),
}
