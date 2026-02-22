#[macro_export]
macro_rules! define_file {
    (
        $name:ident,
        [$($ext:expr),*],
        $($init_bytes:expr)?
    ) => {
        use derive_more::{From, AsRef, Deref, DerefMut};
        
        use crate::{FileBase, FileTrait};
        
        #[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
        #[from(forward)]
        #[as_ref(forward)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            file: FileBase<Self>,
        }
        
        impl $name {
            pub fn new(path: impl AsRef<std::path::Path>) -> Self {
                Self { file: FileBase::new(path) }
            }
        }
        
        impl FileTrait for $name {
            fn ext() -> &'static [&'static str] {
                &[$($ext),*]
            }
            
            $(
                fn file_init_bytes() -> Option<&'static [u8]> {
                    return Some($init_bytes);    
                }
            )?
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
            impl crate::ImageFile for $name {
                fn image_format() -> image::ImageFormat {
                    $format
                }
            }
            
            #[cfg(feature = "async")]
            impl crate::ImageFileAsync for $name {}
        };
    };
}

#[macro_export]
macro_rules! define_custom_quality_image {
    ($name:ident, $config:ident) => {
        #[cfg(feature = "image")]
        const _: () = {
            impl crate::ImageQulityEncoding for $name {
                type Config = $config;
            }

            #[cfg(feature = "async")]
            impl crate::ImageQualityEncodingAsync for $name {}
        };
    };
}
