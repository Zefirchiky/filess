//! File types are convenience enums of all the types by categories.
//!
//! Use this instead of boxed dynamic types.

use crate::{define_file_types, traits::FileTrait};

define_file_types! {
    FileType,
    File,
    "image" Image,
    "just_json" Json,
    "just_toml" Toml,
    "just_ron" Ron,
    "md"   Md,
    "txt"  Txt,
    "jpeg" Jpeg,
    "png"  Png,
    "webp" WebP,
    "gif"  Gif,
    "bmp"  Bmp,
    "exr"  Exr,
    "ff"   Ff,
    "hdr"  Hdr,
    "ico"  Ico,
    "pnm"  Pnm,
    "qoi"  Qoi,
    "tga"  Tga,
    "ogg"  Ogg,
    "mkv"  Mkv,
    "flac" Flac,
    "wav"  Wav,
    "aiff" Aiff,
    "mp4"  Mp4,
    "mp3"  Mp3,
    "mp2"  Mp2,
    "mp1"  Mp1,
    "mpa"  Mpa,
    "alac" Alac,
}

define_file_types! {
    TextTypes,
    File,
    "just_json" Json,
    "just_toml" Toml,
    "just_ron" Ron,
    "md"   Md,
    "txt"  Txt,
}

#[cfg(feature = "_any_model")]
#[derive(Debug, thiserror::Error)]
/// Errors that can occur when working with [ModelType].
pub enum ModelTypeError {
    #[cfg(feature = "json")]
    #[error("Json error: {0}")]
    Json(#[from] <crate::Json as crate::traits::ModelFile>::Error),
    #[cfg(feature = "toml")]
    #[error("Toml error: {0}")]
    Toml(#[from] <crate::Toml as crate::traits::ModelFile>::Error),
    #[cfg(feature = "ron")]
    #[error("Ron error: {0}")]
    Ron(#[from] <crate::Ron as crate::traits::ModelFile>::Error),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(feature = "_any_serde_model")]
impl crate::errors::ModelIoError for ModelTypeError {}

#[cfg(feature = "_any_model")]
#[derive(Debug, Clone)]
/// A dynamic model file that can be Json, Toml, or Ron at runtime.
pub enum ModelType {
    #[cfg(feature = "just_json")]
    Json(crate::Json),
    #[cfg(feature = "just_toml")]
    Toml(crate::Toml),
    #[cfg(feature = "just_ron")]
    Ron(crate::Ron),
}

#[cfg(feature = "_any_model")]
impl FileTrait for ModelType {
    fn _rename_file(&mut self, path: impl AsRef<std::path::Path>) {
        crate::match_self_1_arg!(self, _rename_file, path,
            "just_json" Json,
            "just_toml" Toml,
            "just_ron" Ron,
        );
    }

    fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self::from_ext(path).expect("Must be one of the model formats")
    }

    fn try_new(path: impl AsRef<std::path::Path>) -> Result<Self, Self::TryNewError> {
        Self::from_ext(&path).ok_or(Self::TryNewError::WrongExtension(
            path.as_ref().into(),
            path.as_ref()
                .extension()
                .and_then(|e| Some(e.to_str().unwrap().to_string()))
                .unwrap(),
        ))
    }

    fn ext() -> &'static [&'static str] {
        &[]
    }

    fn ext_name() -> &'static str {
        ""
    }

    fn mime_type() -> &'static [&'static str] {
        &[]
    }
}

#[cfg(feature = "_any_model")]
impl AsRef<std::path::Path> for ModelType {
    fn as_ref(&self) -> &std::path::Path {
        crate::match_self!(self, as_ref,
            "just_json" Json,
            "just_toml" Toml,
            "just_ron" Ron,
        );
    }
}

#[cfg(feature = "_any_model")]
impl AsMut<std::path::Path> for ModelType {
    fn as_mut(&mut self) -> &mut std::path::Path {
        crate::match_self!(self, as_mut,
            "just_json" Json,
            "just_toml" Toml,
            "just_ron" Ron,
        );
    }
}

#[cfg(feature = "_any_model")]
impl Default for ModelType {
    #[allow(unreachable_code)]
    fn default() -> Self {
        #[cfg(feature = "just_json")]
        return Self::Json(crate::Json::default());
        #[cfg(all(feature = "just_toml"))]
        return Self::Toml(crate::Toml::default());
        #[cfg(all(feature = "just_ron"))]
        return Self::Ron(crate::Ron::default());
    }
}

#[cfg(feature = "_any_model")]
impl From<&str> for ModelType {
    fn from(s: &str) -> Self {
        Self::from_ext(s).expect("Must be one of the model formats")
    }
}

#[cfg(feature = "_any_model")]
impl From<std::path::PathBuf> for ModelType {
    fn from(s: std::path::PathBuf) -> Self {
        Self::from_ext(s).expect("Must be one of the model formats")
    }
}

#[cfg(feature = "_any_model")]
impl From<&std::path::Path> for ModelType {
    fn from(s: &std::path::Path) -> Self {
        Self::from_ext(s).expect("Must be one of the model formats")
    }
}

#[cfg(feature = "_any_model")]
impl ModelType {
    /// Creates a [ModelType] from a path by matching its extension.
    ///
    /// Returns [None] if no supported model extension is found.
    #[allow(unused_variables)]
    pub fn from_ext(path: impl AsRef<std::path::Path>) -> Option<Self> {
        let path_ref = path.as_ref();
        if let Some(ext) = path_ref.extension().and_then(|s| s.to_str()) {
            #[cfg(feature = "just_json")]
            if crate::Json::ext().contains(&ext) {
                return Some(Self::Json(crate::Json::new(&path_ref)));
            }
            #[cfg(feature = "just_toml")]
            if crate::Toml::ext().contains(&ext) {
                return Some(Self::Toml(crate::Toml::new(&path_ref)));
            }
            #[cfg(feature = "just_ron")]
            if crate::Ron::ext().contains(&ext) {
                return Some(Self::Ron(crate::Ron::new(&path_ref)));
            }
        }
        None
    }
}

#[cfg(feature = "_any_serde_model")]
impl crate::traits::ModelFile for ModelType {
    type Error = ModelTypeError;

    /// Use [Self::self_model_to_bytes] instead
    fn model_to_bytes(_model: &impl serde::Serialize) -> Result<Vec<u8>, Self::Error> {
        panic!("Use self_model_to_bytes instead")
    }
    fn self_model_to_bytes(&self, model: &impl serde::Serialize) -> Result<Vec<u8>, Self::Error> {
        match self {
            #[cfg(feature = "json")]
            Self::Json(_) => Ok(crate::Json::model_to_bytes(model)?),
            #[cfg(feature = "toml")]
            Self::Toml(_) => Ok(crate::Toml::model_to_bytes(model)?),
            #[cfg(feature = "ron")]
            Self::Ron(_) => Ok(crate::Ron::model_to_bytes(model)?),
        }
    }

    /// Use self_bytes_to_model instead
    fn bytes_to_model<T: for<'de> serde::Deserialize<'de>>(
        _data: Vec<u8>,
    ) -> Result<T, Self::Error> {
        panic!("Use self_bytes_to_model instead")
    }
    fn self_bytes_to_model<T: for<'de> serde::Deserialize<'de>>(
        &self,
        data: Vec<u8>,
    ) -> Result<T, Self::Error> {
        match self {
            #[cfg(feature = "json")]
            Self::Json(_) => Ok(crate::Json::bytes_to_model(data)?),
            #[cfg(feature = "toml")]
            Self::Toml(_) => Ok(crate::Toml::bytes_to_model(data)?),
            #[cfg(feature = "ron")]
            Self::Ron(_) => Ok(crate::Ron::bytes_to_model(data)?),
        }
    }
}

#[cfg(feature = "image")]
define_file_types!(
    ImageTypes,
    Image,
    "jpeg" Jpeg,
    "png"  Png,
    "webp" WebP,
    "gif"  Gif,
    "bmp"  Bmp,
    "exr"  Exr,
    "ff"   Ff,
    "hdr"  Hdr,
    "ico"  Ico,
    "pnm"  Pnm,
    "qoi"  Qoi,
    "tga"  Tga,
);

#[cfg(feature = "image")]
impl crate::traits::ImageFile for ImageTypes {
    fn image_format() -> image::ImageFormat {
        image::ImageFormat::Avif
    }
}

#[cfg(feature = "audio")]
define_file_types!(
    AudioTypes,
    Audio,
    "ogg"  Ogg,
    "mkv"  Mkv,
    "flac" Flac,
    "wav"  Wav,
    "aiff" Aiff,
    "mp4"  Mp4,
    "mp3"  Mp3,
    "mp2"  Mp2,
    "mp1"  Mp1,
    "mpa"  Mpa,
    "alac" Alac,
);

// FIXME: Return type might need to be something like [DynamicReader]
// #[cfg(feature = "audio")]
// impl AudioTypes {
//     fn load_audio(&self) -> Result<crate::DecodedStream<Self, crate::DynamicDecoder>, crate::AudioError> {
//         use crate::{AudioCodecsFile, AudioContainerFile};
//         crate::match_self_wrapped!(self, load_audio,
//             "ogg"  Ogg,
//             "mkv"  Mkv,
//             "flac" Flac,
//             "wav"  Wav,
//             "aiff" Aiff,
//             "mp4"  Mp4,
//             "mp3"  Mp3,
//             "mp2"  Mp2,
//             "mp1"  Mp1,
//             "mpa"  Mpa,
//             "alac" Alac,
//             @Audio,
//         )
//     }
// }

#[cfg(all(test, feature = "json"))]
mod file_types {
    use crate::{FileType, Json};

    #[test]
    fn from_ext() {
        let file = FileType::from_ext("file.json");
        assert_eq!(file, FileType::Json(Json::new(&"file.json")))
    }
}
