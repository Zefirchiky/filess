#[macro_export]
macro_rules! match_self {
    ($self:expr, $action:ident, $($(#[cfg($meta:meta)])? $variant:ident,)*) => {
        match $self {
            $(
                $(#[cfg($meta)])?
                // We bind to 'item' so we can call the method on it
                Self::$variant(item) => return item.$action(),
            )*
        }
    };
}

#[macro_export]
macro_rules! define_file_types {
    (
        $name:ident,
        $fallback:ident,
        $($feature:literal $variant:ident,)*
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name {
            $fallback(crate::$fallback),
            $(
                #[cfg(feature = $feature)]
                $variant(crate::$variant),
            )*
        }
        
        impl FileTrait for $name {
            fn new(file: impl AsRef<std::path::Path>) -> Self {
                Self::from_ext(file)
            }
            
            fn ext() -> &'static [&'static str] {
                &[]
            }
        }
        
        impl AsRef<std::path::Path> for $name {
            fn as_ref(&self) -> &std::path::Path {
                crate::match_self!(self, as_ref, $fallback, $(#[cfg(feature = $feature)] $variant,)*);
            }
        }
        
        $(
            #[cfg(feature = $feature)]
            impl From<crate::$variant> for $name {
                fn from(value: crate::$variant) -> Self {
                    $name::$variant(value)
                }
            }
        )*
        
        impl Default for $name {
            #[allow(unreachable_code)]
            fn default() -> Self {
                $(
                    #[cfg(feature = $feature)]
                    return Self::$variant(crate::$variant::default());
                )*
            }
        }
        
        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self::from_ext(s)
            }
        }
        
        impl From<std::path::PathBuf> for $name {
            fn from(s: std::path::PathBuf) -> Self {
                Self::from_ext(s)
            }
        }
        
        impl From<&std::path::Path> for $name {
            fn from(s: &std::path::Path) -> Self {
                Self::from_ext(s)
            }
        }
        
        impl $name {
            #[allow(unused_variables)]
            pub fn from_ext(path: impl AsRef<std::path::Path>) -> Self {
                let path_ref = path.as_ref();
                #[allow(unused_variables)]
                if let Some(ext) = path_ref.extension().and_then(|s| s.to_str()) {
                    $(
                        #[cfg(feature = $feature)]
                        {
                            if crate::$variant::ext().contains(&ext) {
                                return Self::$variant(crate::$variant::new(&path_ref));
                            }
                        }
                    )*
                }

                // Default fallback
                Self::$fallback(crate::$fallback::new(&path_ref))
            }
        }
    }
}