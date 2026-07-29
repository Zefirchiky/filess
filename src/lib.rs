//! Simplified file primitives.
//!
//! `filess` should be used to enforce file types needed.
//!
//! It was designed to be lightweight, with all integrations being optional.
//!
//! Each file type has it's own feature, which is the reason for so many feature flags.
#![deny(unreachable_pub)]
#![allow(refining_impl_trait, async_fn_in_trait)]
mod dir;
mod dir_file;
pub mod errors;
mod file_bases;
pub mod file_types;
pub mod files;
mod macros;
#[cfg(feature = "open")]
mod open_integration;
pub mod primitives;
mod temporary;
pub mod traits;

pub use dir::{Dir, DirAny};
pub use dir_file::{DirFile, DirFileAny};
#[cfg(feature = "audio")]
pub use file_bases::audio_file::DecodedStream;
pub use file_types::*;
pub use files::*;
pub use temporary::Temporary;

#[cfg(feature = "infer")]
pub use infer;
#[cfg(feature = "open")]
pub use open;
#[cfg(feature = "trash")]
pub use trash;
#[cfg(feature = "walk")]
pub use walkdir;
#[cfg(feature = "glob")]
pub use glob;

#[cfg(test)]
pub(crate) mod test_assets {
    #[allow(unused)]
    pub(crate) fn get_temp_path(name: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("{}_{}.json", name, now));
        path
    }

    #[cfg_attr(
        feature = "serde",
        derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)
    )]
    #[allow(unused)]
    pub(crate) struct User {
        pub name: String,
        pub age: usize,
    }
}
