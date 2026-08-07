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
macro_rules! match_self_1_arg {
    ($self:expr, $action:ident, $argument:ident, $( $($feature:literal)? $variant:ident, )* $(@ $($panic_feature:literal)? $panic_var:ident, )* ) => {
        match $self {
            $(
                $(#[cfg(feature = $feature)])?
                Self::$variant(item) => return item.$action($argument),
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
                Self::$variant(item) => return Self::$variant(item.$action().unwrap()), // FIXME: No unwrap
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
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum $name {
            $fallback($crate::$fallback),
            $(
                #[cfg(feature = $feature)]
                $variant($crate::$variant),
            )*
        }

        impl FileTrait for $name {
            fn try_new(file: impl AsRef<std::path::Path>) -> Result<Self, Self::TryNewError> {
                Ok(Self::from_ext(file))
            }
            
            fn _rename_file(&mut self, path: impl AsRef<std::path::Path>) {
                let path: &std::path::Path = path.as_ref().into();
                $crate::match_self_1_arg!(self, _rename_file, path, $fallback, $($feature $variant,)*);
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

        impl AsRef<std::path::Path> for $name {
            fn as_ref(&self) -> &std::path::Path {
                $crate::match_self!(self, as_ref, $fallback, $($feature $variant,)*);
            }
        }

        impl AsMut<std::path::Path> for $name {
            fn as_mut(&mut self) -> &mut std::path::Path {
                $crate::match_self!(self, as_mut, $fallback, $($feature $variant,)*);
            }
        }

        $(
            #[cfg(feature = $feature)]
            impl From<$crate::$variant> for $name {
                fn from(value: $crate::$variant) -> Self {
                    $name::$variant(value)
                }
            }
        )*

        impl Default for $name {
            #[allow(unreachable_code)]
            fn default() -> Self {
                $(
                    #[cfg(feature = $feature)]
                    return Self::$variant($crate::$variant::default());
                    return Self::$fallback($crate::$fallback::default());
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
                            if $crate::$variant::ext().contains(&ext) {
                                return Self::$variant($crate::$variant::new(&path_ref));
                            }
                        }
                    )*
                }

                // Default fallback
                Self::$fallback($crate::$fallback::new(&path_ref))
            }
        }
    }
}
