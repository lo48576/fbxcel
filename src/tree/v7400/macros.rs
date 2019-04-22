//! Macros.

/// Constructs a tree.
///
/// Enabled by `tree` feature.
///
/// # Examples
///
/// ```
/// # use fbxcel::tree_v7400;
/// let empty_tree = tree_v7400! {};
/// ```
///
/// ```
/// # use fbxcel::tree_v7400;
/// let tree = tree_v7400! {
///     Node0: {
///         Node0_0: {}
///         Node0_1: {}
///     }
///     Node1: {
///         // You can use trailing comma.
///         Node1_0: {},
///         Node1_1: {},
///     }
///     // Use parens to specify attributes by single array.
///     // Note that the expression inside parens should implement
///     // `IntoIterator<Item = AttributeValue>`.
///     Node2: (vec!["hello".into(), "world".into(), 42i32.into()]) {}
///     // Use brackets to specify attributes one by one.
///     Node3: ["hello", "world", 3.14f32, &b"BINARY"[..]] {}
/// };
/// ```
#[macro_export]
macro_rules! tree_v7400 {
    (@__node, $tree:ident, $parent:ident,) => {};
    (@__node, $tree:ident, $parent:ident, , $($rest:tt)*) => {
        tree_v7400! { @__node, $tree, $parent, $($rest)* }
    };

    (@__node, $tree:ident, $parent:ident,
        $name:ident: {
            $($subtree:tt)*
        }
        $($rest:tt)*
    ) => {{
        {
            let _node = $tree.append_new($parent, stringify!($name));
            tree_v7400! { @__node, $tree, _node, $($subtree)* }
        }
        tree_v7400! { @__node, $tree, $parent, $($rest)* }
    }};
    (@__node, $tree:ident, $parent:ident,
        $name:ident: [$($attr:expr),* $(,)?] {
            $($subtree:tt)*
        }
        $($rest:tt)*
    ) => {{
        {
            let _node = $tree.append_new($parent, stringify!($name));
            $(
                $tree.append_attribute(_node, $attr);
            )*
            tree_v7400! { @__node, $tree, _node, $($subtree)* }
        }
        tree_v7400! { @__node, $tree, $parent, $($rest)* }
    }};
    (@__node, $tree:ident, $parent:ident,
        $name:ident: ($attrs:expr) {
            $($subtree:tt)*
        }
        $($rest:tt)*
    ) => {{
        {
            let _node = $tree.append_new($parent, stringify!($name));
            $attrs.into_iter().for_each(|attr: $crate::low::v7400::AttributeValue| $tree.append_attribute(_node, attr));
            tree_v7400! { @__node, $tree, _node, $($subtree)* }
        }
        tree_v7400! { @__node, $tree, $parent, $($rest)* }
    }};

    ($($rest:tt)*) => {
        {
            #[allow(unused_mut)]
            let mut tree = $crate::tree::v7400::Tree::default();
            let _root = tree.root().node_id();
            tree_v7400! { @__node, tree, _root, $($rest)* }
            tree
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_tree() {
        let _ = tree_v7400! {};
    }

    #[test]
    fn empty_node() {
        let _ = tree_v7400! {
            Hello: {}
            World: {},
        };
    }

    #[test]
    fn nested_node() {
        let _ = tree_v7400! {
            Hello: {
                Hello1: {},
                Hello2: {}
            }
            World: {
                World1: {
                    World1_1: {}
                    World1_2: {}
                }
                World2: {},
            },
        };
    }

    #[test]
    fn nested_node_with_attrs() {
        let _ = tree_v7400! {
            Hello: {
                Hello1: (vec!["string".into()]) {},
                Hello2: [3.14f32, 42i64] {}
            }
            World: {
                World1: {
                    World1_1: (vec!["Hello".into(), 42i32.into()]) {}
                    World1_2: [] {}
                }
                World2: {},
            },
        };
    }
}
