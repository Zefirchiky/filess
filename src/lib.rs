#![deny(unreachable_pub)]
#![allow(refining_impl_trait, async_fn_in_trait)]
#[cfg(feature = "audio")]
mod audio_file;
mod dir;
pub mod errors;
mod file_base;
mod file_macros;
mod file_type_macros;
pub mod file_types;
pub mod files;
mod fs_handler;
#[cfg(feature = "image")]
mod image_file;
#[cfg(feature = "serde")]
mod model_file;
#[cfg(feature = "open")]
mod open_integration;
pub mod primitives;
pub mod traits;
mod temporary;
mod util_macros;

#[cfg(feature = "audio")]
pub use audio_file::DecodedStream;
pub use dir::Dir;
pub use file_types::*;
pub use files::*;
pub use fs_handler::FsHandler;
pub use temporary::Temporary;

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
