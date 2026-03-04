#[macro_export]
macro_rules! match_self {
    ($self:expr, $action:ident, $( $($feature:literal)? $variant:ident, )* $(@ $($panic_feature:literal)? $panic_var:ident, )* ) => {
        match $self {
            $(
                $(#[cfg(feature = $feature)])?
                Self::$variant(item) => return item.$action(),
            )*
            $(
                $(#[cfg(feature = $panic_feature)])?
                Self::$panic_var(_) => panic!("Operation on this type is not supported"),
            )*
        }
    };
}

#[macro_export]
macro_rules! match_self_wrapped {
    ($self:expr, $action:ident, $( $($feature:literal)? $variant:ident, )* $(@ $($panic_feature:literal)? $panic_var:ident, )* ) => {
        match $self {
            $(
                $(#[cfg(feature = $feature)])?
                Self::$variant(item) => return Self::$variant(item.$action()?),
            )*
            $(
                $(#[cfg(feature = $panic_feature)])?
                Self::$panic_var(_) => panic!("Operation on this type is not supported"),
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
                crate::match_self!(self, as_ref, $fallback, $($feature $variant,)*);
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
                    return Self::$fallback(crate::$fallback::default());
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
