#![allow(refining_impl_trait, async_fn_in_trait)]
mod dir;
mod file_base;
mod file_types;
pub mod files;
mod fs_handler;
#[cfg(feature = "image")]
mod image_file;
#[cfg(feature = "serde")]
mod model_file;

pub use dir::Dir;
pub use file_base::{FileBase, FileTrait, Temporary};
pub use file_types::*;
pub use files::*;
pub use fs_handler::FsHandler;
#[cfg(feature = "image")]
pub use image_file::*;
#[cfg(feature = "serde")]
pub use model_file::*;

#[cfg(test)]
pub mod test_assets {
    pub fn get_temp_path(name: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        path.push(format!("{}_{}.json", name, now));
        path
    }
    
    #[cfg_attr(feature = "serde", derive(Debug, serde::Serialize, serde::Deserialize, PartialEq))]
    pub struct User {
        pub name: String,
        pub age: usize,
    }
}
