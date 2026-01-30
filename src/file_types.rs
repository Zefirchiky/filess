use crate::{File, Json, Md};

#[derive(Debug)]
pub enum FileTypes {
    File(File),
    Json(Json),
    Md(Md),
}
