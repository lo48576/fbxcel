//! Utility macros.

/// Defines a new node ID type, and implements some traits automatically.
macro_rules! define_node_id_type {
    (
        $(#[$meta:meta])*
        $ty_id:ident {
            // Ancestors, in descendants-to-ancestors order.
            ancestors {
                $ty_parent:ty $(, $ty_ancestor:ty),* $(,)?
            }
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $(#[$meta])*
        pub struct $ty_id($ty_parent);

        impl $ty_id {
            /// Creates a new node ID object.
            pub(crate) fn new(id: $ty_parent) -> Self {
                $ty_id(id)
            }
        }

        impl From<$ty_id> for $ty_parent {
            fn from(v: $ty_id) -> Self {
                v.0
            }
        }

        impl $crate::dom::v7400::DowncastId<$ty_id> for $ty_parent {
            fn downcast(self, doc: &$crate::dom::v7400::Document) -> Option<$ty_id> {
                trace!(
                    concat!(
                        "Trying to downcast {:?} to `",
                        stringify!($ty_id),
                        "`"
                    ),
                    self
                );
                let maybe_invalid_id = $ty_id::new(self);
                let is_valid = $crate::dom::v7400::ValidateId::validate_id(maybe_invalid_id, doc);
                if is_valid {
                    // Valid!
                    trace!(
                        "Successfully downcasted {:?} to {:?}",
                        self,
                        maybe_invalid_id
                    );
                    Some(maybe_invalid_id)
                } else {
                    // Invalid.
                    trace!(
                        concat!(
                            "Downcast failed: {:?} is not convertible to `",
                            stringify!($ty_id),
                            "`"
                        ),
                        self
                    );
                    None
                }
            }
        }

        define_node_id_type! { @ancestors_transitive, $ty_id, $ty_parent, $($ty_ancestor,)* }
    };
    (@ancestors_transitive, $ty_id:ident, $ty_ancestor:ty, $ty_ancestor_parent:ty, $($rest:ty,)*) => {
        impl From<$ty_id> for $ty_ancestor_parent {
            fn from(v: $ty_id) -> Self {
                let ancestor_id: $ty_ancestor = v.into();
                ancestor_id.into()
            }
        }
        define_node_id_type! { @ancestors_transitive, $ty_id, $ty_ancestor_parent, $($rest,)* }
    };
    (@ancestors_transitive, $ty_id:ident, $ty_ancestor:ty,) => {};
}
