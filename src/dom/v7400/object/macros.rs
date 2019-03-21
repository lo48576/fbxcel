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
