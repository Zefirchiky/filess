#[macro_export]
macro_rules! define_file {
    (
        $name:ident,
        [$($ext:expr),*]
        $(,$init_bytes:expr)?
    ) => {
        use derive_more::{From, AsRef, Deref, DerefMut};

        pub use crate::{FileBase, FileTrait};

        #[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut, PartialEq)]
        #[from(forward)]
        #[as_ref(forward)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[doc = concat!("Returns the file extensions supported by ", stringify!($name), ".")]
        pub struct $name {
            file: FileBase<Self>,
        }

        impl $name {
            #[doc = concat!("Creates new ", stringify!($name), ".")]
            pub fn new(path: impl AsRef<std::path::Path>) -> Self { // A convenience method, otherwise user will need to import `FileTrait`
                <Self as FileTrait>::new(path)      // ? : Duplication that might be unnecessary???
                // TODO: Check binary size generated
            }
        }

        impl FileTrait for $name {
            #[doc = concat!("Creates new ", stringify!($name), ".",
                "\n\n#Panics")]
            fn new(path: impl AsRef<std::path::Path>) -> Self {
                Self { file: FileBase::new(path) }
            }

            #[doc = concat!("Returns the file extensions supported by ", stringify!($name), ".")]
            fn ext() -> &'static [&'static str] {
                &[$($ext),*]
            }

            $(
                #[doc = concat!("Returns optional file initial bytes for ", stringify!($name), ".")]
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
