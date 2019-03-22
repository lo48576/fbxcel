//! Macros.

macro_rules! define_object_subtype {
    (
        $(#[$meta:meta])*
        $ty_sub:ident: $ty_super:ident
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy)]
        pub struct $ty_sub<'a> {
            /// Object handle.
            object: $ty_super<'a>,
        }

        impl<'a> $ty_sub<'a> {
            /// Creates a new handle.
            pub(crate) fn new(object: $ty_super<'a>) -> Self {
                Self { object }
            }
        }

        impl<'a> std::ops::Deref for $ty_sub<'a> {
            type Target = $ty_super<'a>;

            fn deref(&self) -> &Self::Target {
                &self.object
            }
        }
    }
}

macro_rules! define_typed_handle {
    (
        $(#[$outer_meta:meta])*
        $outer:ident($inner_def:ident) {
            $(
                $(#[$variant_meta:meta])*
                // I want `$inner` to match type path without generic parameter
                // (e.g. `std::vec::Vec`) and use it like `$inner<'a>`, but it
                // seems impossible for now.
                ($class:pat, $subclass:pat) => $variant:ident($inner:ident),
            )*
        }
    ) => {
        $(#[$outer_meta])*
        #[derive(Debug, Clone, Copy)]
        pub enum $outer<'a> {
            $(
                $(#[$variant_meta])*
                $variant($inner<'a>),
            )*
            /// Unkonwn.
            Unknown($inner_def<'a>),
            #[doc(hidden)]
            __Nonexhaustive,
        }

        impl<'a> $outer<'a> {
            /// Creates a new handle from the given object handle.
            pub(crate) fn new(obj: $inner_def<'a>) -> Self {
                match (obj.class(), obj.subclass()) {
                    $(
                        ($class, $subclass) => $outer::$variant(<$inner>::new(obj)),
                    )*
                    _ => $outer::Unknown(obj),
                }
            }
        }

        impl<'a> std::ops::Deref for $outer<'a> {
            type Target = $inner_def<'a>;

            fn deref(&self) -> &Self::Target {
                match self {
                    $(
                        $outer::$variant(o) => &**o,
                    )*
                    $outer::Unknown(o) => o,
                    $outer::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
                }
            }
        }
    };
}
