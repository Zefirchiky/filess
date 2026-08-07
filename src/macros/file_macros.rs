#[macro_export]
macro_rules! define_file {
    (
        $name:ident,
        $ext_name:expr,
        [$($mimo:expr),*],
        [$($ext:expr),*]
        $(,$init_bytes:expr)?
    ) => {
        use $crate::{primitives::FileBase, traits::FileTrait};

        #[derive(Debug, Default, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            file: FileBase<Self>,
        }

        impl $name {
            #[doc = concat!("Creates new ", stringify!($name), ".")]
            pub fn try_new(path: impl AsRef<std::path::Path>) -> Result<Self, <Self as $crate::traits::FsElement>::TryNewError> { // A convenience method, otherwise user will need to import [FileTrait]
                <Self as FileTrait>::try_new(path)      // ? : Duplication that might be unnecessary???
                // TODO: Check binary size generated
            }
            
            #[doc = concat!("Creates new ", stringify!($name), ".")]
            pub fn new(path: impl AsRef<std::path::Path>) -> Self { // A convenience method, otherwise user will need to import [FileTrait]
                <Self as FileTrait>::new(path)      // ? : Duplication that might be unnecessary???
                // TODO: Check binary size generated
            }
        }

        impl FileTrait for $name {
            #[doc = concat!("Creates new ", stringify!($name), ".",
                "\n\n#Panics")]
            fn try_new(path: impl AsRef<std::path::Path>) -> Result<Self, Self::TryNewError> {
                Ok(Self { file: FileBase::try_new(path)? })
            }
            
            fn _rename_file(&mut self, path: impl AsRef<std::path::Path>) {
                self.file.path = path.as_ref().into()
            }

            #[doc = concat!("Returns the file extensions supported by ", stringify!($name), ".")]
            fn ext() -> &'static [&'static str] {
                &[$($ext),*]
            }

            fn ext_name() -> &'static str { $ext_name }
            fn mime_type() -> &'static [&'static str] {
                &[$($mimo),*]
            }

            $(
                #[doc = concat!("Returns optional file initial bytes for ", stringify!($name), ".")]
                fn file_init_bytes() -> Option<&'static [u8]> {
                    return Some($init_bytes);
                }
            )?
        }

        impl AsRef<std::path::Path> for $name {
            fn as_ref(&self) -> &std::path::Path {
                &self
            }
        }

        impl AsMut<std::path::Path> for $name {
            fn as_mut(&mut self) -> &mut std::path::Path {
                &mut self.file
            }
        }

        impl From<&FileBase<Self>> for $name {
            fn from(path: &FileBase<Self>) -> Self {
                Self::new(path)
            }
        }

        impl From<&std::path::Path> for $name {
            fn from(path: &std::path::Path) -> Self {
                Self::new(path)
            }
        }

        impl From<std::path::PathBuf> for $name {
            fn from(path: std::path::PathBuf) -> Self {
                Self::new(path)
            }
        }

        impl From<&str> for $name {
            fn from(path: &str) -> Self {
                Self::new(path)
            }
        }

        impl From<String> for $name {
            fn from(path: String) -> Self {
                Self::new(path)
            }
        }

        impl std::ops::Deref for $name {
            type Target = FileBase<Self>;
            fn deref(&self) -> &Self::Target {
                &self.file
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.file
            }
        }

        #[cfg(all(test, feature = "__full_test"))]
        #[allow(non_snake_case)]
        mod __file_tests {
            use super::*;
            use $crate::{Temporary, traits::FsElement};
            use std::path::{Path, PathBuf};

            fn __base(label: &str) -> PathBuf {
                let dir = std::env::temp_dir().join("filess_auto_test");
                let exts = <$name as FileTrait>::ext();
                if exts.is_empty() {
                    dir.join(label)
                } else if exts[0].is_empty() {
                    // ext = [""], produce "label." so extension is Some("")
                    dir.join(format!("{}.", label))
                } else {
                    dir.join(format!("{}.{}", label, exts[0]))
                }
            }

            #[test]
            fn ext_non_empty() {
                assert!(!<$name as FileTrait>::ext().is_empty());
            }

            #[test]
            fn try_new_valid() {
                <$name as FileTrait>::try_new(&__base("valid")).unwrap();
            }

            #[test]
            fn try_new_invalid_extension() {
                let err = <$name as FileTrait>::try_new("data.invalid").unwrap_err();
                let msg = err.to_string();
                assert!(msg.contains("invalid") || msg.contains("no extension") || msg.contains("UTF-8"));
            }

            #[test]
            fn create_and_remove() {
                let p = __base("cr");
                let f = Temporary::new(<$name as FileTrait>::new(&p));
                f.create().unwrap();
                assert!(p.exists());
                f.remove().unwrap();
                assert!(!p.exists());
            }

            #[test]
            fn save_and_load() {
                let p = __base("sl");
                let f = Temporary::new(<$name as FileTrait>::new(&p));
                let data = b"auto test data";
                f.save(data).unwrap();
                assert_eq!(f.load().unwrap(), data);
            }

            #[test]
            fn from_path() {
                let _f = <$name as From<&Path>>::from(&__base("fp"));
            }

            #[test]
            fn from_pathbuf() {
                let _f = <$name as From<PathBuf>>::from(__base("fpb"));
            }

            #[test]
            fn from_str() {
                let s = __base("fs").to_string_lossy().to_string();
                let _f = <$name as From<&str>>::from(&s as &str);
            }

            #[test]
            fn from_string() {
                let s = __base("fstr").to_string_lossy().to_string();
                let _f = <$name as From<String>>::from(s);
            }

            #[test]
            fn copy_roundtrip() {
                let src = __base("copy_src");
                let dst = __base("copy_dst");
                let f = Temporary::new(<$name as FileTrait>::new(&src));
                f.save(b"copy data").unwrap();
                let copied = f.copy(&dst).unwrap();
                assert!(dst.exists());
                assert_eq!(copied.load().unwrap(), b"copy data");
            }

            #[test]
            fn rename_fs_and_inner() {
                let src = __base("ren_src");
                let dst = __base("ren_dst");
                let mut f = Temporary::new(<$name as FileTrait>::new(&src));
                f.save(b"rename data").unwrap();
                f.rename(&dst).unwrap();
                assert!(!src.exists());
                assert!(dst.exists());
                assert_eq!(f.load().unwrap(), b"rename data");
            }

            #[cfg(feature = "infer")]
            #[test]
            fn infer_does_not_error() {
                let p = __base("infer");
                let f = Temporary::new(<$name as FileTrait>::new(&p));
                let _ = f.infer();
            }
        }
    };
}

#[macro_export]
macro_rules! define_image_file {
    (
        $name:ident,
        $format:expr
    ) => {
        #[cfg(feature = "image")]
        const _: () = {
            impl $crate::traits::ImageFile for $name {
                fn image_format() -> image::ImageFormat {
                    $format
                }
            }
        };
    };
}

#[macro_export]
macro_rules! define_custom_quality_image {
    ($name:ident, $config:ident) => {
        #[cfg(feature = "image")]
        const _: () = {
            impl $crate::traits::ImageQualityEncoding for $name {
                type Config = $config;
            }
        };
    };
}

#[macro_export]
macro_rules! define_audio_file {
    ($name:ident, $reader:ident) => {
        #[cfg(feature = "audio")]
        impl $crate::traits::AudioFile for $name {
            type Reader = symphonia::default::formats::$reader;
        }
    };
}

#[macro_export]
macro_rules! define_audio_codecs_file {
    ($name:ident, $decoder:ident, $codecs_type:ident) => {
        #[cfg(feature = "audio")]
        impl $crate::traits::AudioCodecsFile for $name {
            type Decoder = symphonia::default::codecs::$decoder;
            fn codec_type() -> symphonia::core::codecs::CodecType { symphonia::core::codecs::$codecs_type }
        }
    };
}

#[macro_export]
macro_rules! define_audio_container_file {
    ($name:ident) => {
        #[cfg(feature = "audio")]
        impl $crate::traits::AudioContainerFile for $name {}
    };
}


