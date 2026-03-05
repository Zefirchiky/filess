use crate::{FileTrait, define_file_types};

define_file_types! {
    FileType,
    File,
    "image" Image,
    "json" Json,
    "toml" Toml,
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
}

define_file_types! {
    TextTypes,
    File,
    "json" Json,
    "toml" Toml,
    "md"   Md,
    "txt"  Txt,
}

#[cfg(feature = "_any_serde_model")]
#[derive(Debug, thiserror::Error)]
pub enum ModelTypeError {
    #[cfg(feature = "serde_json")]
    #[error("Json error")]
    Json(#[from] <crate::Json as crate::ModelFile>::Error),
    #[cfg(feature = "serde_toml")]
    #[error("Toml error")]
    Toml(#[from] <crate::Toml as crate::ModelFile>::Error),
    #[error("Io error")]
    Io(#[from] std::io::Error),
}

#[cfg(feature = "_any_serde_model")]
impl crate::ModelIoError for ModelTypeError {}

#[cfg(feature = "_any_serde_model")]
#[derive(Debug, Clone)]
pub enum ModelType {
    #[cfg(feature = "serde_json")]
    Json(crate::Json),
    #[cfg(feature = "serde_toml")]
    Toml(crate::Toml),
}

#[cfg(feature = "_any_serde_model")]
impl FileTrait for ModelType {
    fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self::from_ext(path).expect("Must be one of the model formats")
    }

    fn ext() -> &'static [&'static str] {
        &[]
    }
}

#[cfg(feature = "_any_serde_model")]
impl AsRef<std::path::Path> for ModelType {
    fn as_ref(&self) -> &std::path::Path {
        crate::match_self!(self, as_ref,
            "json" Json,
            "toml" Toml,
        );
    }
}

#[cfg(feature = "_any_serde_model")]
impl Default for ModelType {
    fn default() -> Self {
        #[cfg(feature = "json")]
        return Self::Json(crate::Json::default());
        #[cfg(all(feature = "toml", not(feature = "json")))]
        return Self::Toml(crate::Toml::default());
    }
}

#[cfg(feature = "_any_serde_model")]
impl From<&str> for ModelType {
    fn from(s: &str) -> Self {
        Self::from_ext(s).expect("Must be one of the model formats")
    }
}

#[cfg(feature = "_any_serde_model")]
impl From<std::path::PathBuf> for ModelType {
    fn from(s: std::path::PathBuf) -> Self {
        Self::from_ext(s).expect("Must be one of the model formats")
    }
}

#[cfg(feature = "_any_serde_model")]
impl From<&std::path::Path> for ModelType {
    fn from(s: &std::path::Path) -> Self {
        Self::from_ext(s).expect("Must be one of the model formats")
    }
}

#[cfg(feature = "_any_serde_model")]
impl ModelType {
    #[allow(unused_variables)]
    pub fn from_ext(path: impl AsRef<std::path::Path>) -> Option<Self> {
        #[cfg(feature = "_any_model")]
        {
            let path_ref = path.as_ref();
            if let Some(ext) = path_ref.extension().and_then(|s| s.to_str()) {
                #[cfg(feature = "json")]
                if crate::Json::ext().contains(&ext) {
                    return Some(Self::Json(crate::Json::new(&path_ref)));
                }
                #[cfg(feature = "toml")]
                if crate::Toml::ext().contains(&ext) {
                    return Some(Self::Toml(crate::Toml::new(&path_ref)));
                }
            }
        }
        None
    }
}

#[cfg(feature = "_any_serde_model")]
impl crate::ModelFile for ModelType {
    type Error = ModelTypeError;

    /// Use `self_model_to_bytes` instead
    fn model_to_bytes(_model: &impl serde::Serialize) -> Result<Vec<u8>, Self::Error> {
        panic!("Use self_model_to_bytes instead")
    }
    fn self_model_to_bytes(&self, model: &impl serde::Serialize) -> Result<Vec<u8>, Self::Error> {
        match self {
            #[cfg(feature = "serde_json")]
            Self::Json(_) => Ok(crate::Json::model_to_bytes(model)?),
            #[cfg(feature = "serde_toml")]
            Self::Toml(_) => Ok(crate::Toml::model_to_bytes(model)?),
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
            #[cfg(feature = "serde_json")]
            Self::Json(_) => Ok(crate::Json::bytes_to_model(data)?),
            #[cfg(feature = "serde_toml")]
            Self::Toml(_) => Ok(crate::Toml::bytes_to_model(data)?),
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
impl crate::ImageFile for ImageTypes {
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

// FIXME: Return type might need to be something like `DynamicReader`
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
