//! Simplified file primitives.
//!
//! Use `filess` to enforce file types at the type level — each format is a type (e.g. [Json],
//! [Png], [Flac]). All integrations are optional behind feature flags.
//!
//! # Supported formats (30 total)
//!
//! **Text:** [Json], [Toml], [Ron], [Md], [Txt]
//!
//! **Image:** [Jpeg], [Png], [WebP], [Avif], [Tiff], [Gif], [Bmp], [Exr], [Ff], [Hdr], [Ico],
//! [Pnm], [Qoi], [Tga]
//!
//! **Audio:** [Ogg], [Mkv], [Flac], [Wav], [Aiff], [Mp4], [Mp3], [Mp2], [Mp1], [Mpa], [Alac]
//!
//! **Combined:** [File], [Image], [Audio], [FileType], [TextTypes], [ImageTypes], [AudioTypes],
//! [ModelType]
//!
//! # Integrations
//!
//! | Integration | Feature | What it adds |
//! |-------------|---------|-------------|
//! | **Serde** | `serde` | [ModelFile](traits::ModelFile) — `save_model`, `load_model` |
//! | **Image** | `image` | [ImageFile](traits::ImageFile) — `save_image`, `load_image` + custom quality for [Jpeg], [Png], [Gif], [Avif] |
//! | **Audio** | `audio` | [AudioFile](traits::AudioFile) — `load_audio` via symphonia |
//! | **Async** | `async` | `a`-prefixed variants of all methods + [Temporary] offload |
//! | **Open** | `open` | [OpenTrait](traits::OpenTrait) — `open`, `open_with` |
//! | **Infer** | `infer` | [FileTrait](traits::FileTrait) — `infer`, `enforce` data correctness |
//! | **Walk** | `walk` | [Dir] — `walk` directories |
//! | **Glob** | `glob` | [Dir] — `glob`, `glob_with` |
//! | **Trash** | `trash` | [FsElement](traits::FsElement) — `trash` files/dirs |
//!
//! # Types
//!
//! [Temporary]\<F\> — auto-cleaning wrapper that deletes the file (and empty parent dirs) on drop.
//!
//! [Dir]\<F\> — directory containing multiple files. Supports load/save all, glob, walk, async.
//!
//! [FileType], [ModelType], [TextTypes], [ImageTypes], [AudioTypes] — enums over all types in each
//! category for runtime dispatch without boxing.
//!
//! # Usage
//!
//! ```ignore
//! let json = Json::new("path/to/file.json");
//! let data: Vec<u8> = json.load()?;
//! let model: MyModel = json.load_model()?;        // serde
//! json.save(&data)?;
//! json.save_model(&model)?;                        // serde
//!
//! let img = Temporary::new(Jpeg::new("img.jpg"));  // auto-deleted on drop
//! let image = img.load_image()?;                   // image
//! img.save_image(&image)?;
//! img.save_image_custom(&image, JpegConfig { quality: 40 })?;
//!
//! let audio = Flac::new("track.flac").load_audio()?; // symphonia
//! let stream: DecodedStream = audio;
//!
//! // Each fn has async variants (prefixed with `a`):
//! Jpeg::new("img.jpg").asave_image(&image).await?;
//! ```
#![deny(unreachable_pub)]
#![allow(refining_impl_trait, async_fn_in_trait)]
mod dirs;
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

pub use dirs::*;
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
