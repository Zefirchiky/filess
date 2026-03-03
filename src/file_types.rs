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

#[cfg(all(feature = "serde", feature = "_any_model"))]
#[derive(Debug, thiserror::Error)]
pub enum ModelTypeError {
    #[error("Json error")]
    Json(#[from] <crate::Json as crate::ModelFile>::Error),
    #[error("Toml error")]
    Toml(#[from] <crate::Toml as crate::ModelFile>::Error),
    #[error("Io error")]
    Io(#[from] std::io::Error),
}

#[cfg(all(feature = "serde", feature = "_any_model"))]
impl crate::ModelIoError for ModelTypeError {}

#[cfg(all(feature = "serde", feature = "_any_model"))]
#[derive(Debug, Clone)]
pub enum ModelType {
    #[cfg(feature = "json")]
    Json(crate::Json),
    #[cfg(feature = "toml")]
    Toml(crate::Toml),
}
    
#[cfg(all(feature = "serde", feature = "_any_model"))]
impl FileTrait for ModelType {
    fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self::from_ext(path).expect("Must be one of the model formats")
    }
    
    fn ext() -> &'static [&'static str] {
        &[]
    }
}
    
#[cfg(all(feature = "serde", feature = "_any_model"))]
impl AsRef<std::path::Path> for ModelType {
    fn as_ref(&self) -> &std::path::Path {
        crate::match_self!(self, as_ref, 
            #[cfg(feature = "json")] Json,
            #[cfg(feature = "toml")] Toml,
        );
    }
}

#[cfg(all(feature = "serde", feature = "_any_model"))]
impl Default for ModelType {
    fn default() -> Self {
        #[cfg(feature = "json")]
        return Self::Json(crate::Json::default());
        #[cfg(all(feature = "toml", not(feature = "json")))]
        return Self::Toml(crate::Toml::default());
    }
}

#[cfg(all(feature = "serde", feature = "_any_model"))]
impl From<&str> for ModelType {
    fn from(s: &str) -> Self {
        Self::from_ext(s).expect("Must be one of the model formats")
    }
}

#[cfg(all(feature = "serde", feature = "_any_model"))]
impl From<std::path::PathBuf> for ModelType {
    fn from(s: std::path::PathBuf) -> Self {
        Self::from_ext(s).expect("Must be one of the model formats")
    }
}

#[cfg(all(feature = "serde", feature = "_any_model"))]
impl From<&std::path::Path> for ModelType {
    fn from(s: &std::path::Path) -> Self {
        Self::from_ext(s).expect("Must be one of the model formats")
    }
}

#[cfg(all(feature = "serde", feature = "_any_model"))]
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

#[cfg(all(feature = "serde", feature = "_any_model"))]
impl crate::ModelFile for ModelType {
    type Error = ModelTypeError;
    
    fn model_to_bytes(&self, model: &impl serde::Serialize) -> Result<Vec<u8>, Self::Error> {
        match self {
            Self::Json(m) => Ok(m.model_to_bytes(model)?),
            Self::Toml(m) => Ok(m.model_to_bytes(model)?),
        }
    }
    
    fn bytes_to_model<T: for<'de> serde::Deserialize<'de>>(&self, data: Vec<u8>) -> Result<T, Self::Error> {
        match self {
            Self::Json(m) => Ok(m.bytes_to_model(data)?),
            Self::Toml(m) => Ok(m.bytes_to_model(data)?),
        }
    }
}

// // impl crate::ModelFile for ModelFile {

// // }

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



#[cfg(all(test, feature = "json"))]
mod file_types {
    use crate::{FileType, Json};

    #[test]
    fn from_ext() {
        let file = FileType::from_ext("file.json");
        assert_eq!(file, FileType::Json(Json::new(&"file.json")))
    }
}
