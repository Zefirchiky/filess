use crate::{Dir, FileType};

pub enum FsHandler {
    Dir(Dir),
    File(FileType),
}
