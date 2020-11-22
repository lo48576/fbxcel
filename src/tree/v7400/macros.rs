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
///     Node3: ["hello", "world", 1.234f32, &b"BINARY"[..]] {}
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
    use crate::tree::v7400::Tree;

    #[test]
    fn empty_trees_eq() {
        let tree1 = tree_v7400! {};
        let tree2 = tree_v7400! {};
        assert!(tree1.strict_eq(&tree2));
    }

    #[test]
    fn empty_nodes() {
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
                Hello2: [1.234f32, 42i64] {}
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

    #[test]
    fn compare_complex_trees() {
        fn gentree() -> Tree {
            tree_v7400! {
                Node0: {},
                Node1: {
                    Node1_0: {},
                    Node1_1: {},
                    Node1_2: {
                        Node1_2_child: {},
                        Node1_2_child: {},
                    },
                },
                Node2: [true, 42i16, 42i32, 42i64, 1.414f32, 1.234f64] {
                    Node2_0: (vec![vec![true, false].into(), vec![0i32, 42i32].into()]) {},
                    Node2_1: [
                        vec![std::f32::NAN, std::f32::INFINITY],
                        vec![std::f64::NAN, std::f64::INFINITY]
                    ] {},
                },
            }
        }
        let tree1 = gentree();
        let tree2 = gentree();
        assert!(tree1.strict_eq(&tree2));
        let empty = tree_v7400!();
        assert!(!empty.strict_eq(&tree2));
    }

    #[test]
    fn correct_tree() {
        let tree_manual = {
            let mut tree = Tree::default();
            // Node 0: Without attributes, with children.
            // Node 0-x: Without attributes, without children.
            let node0_id = tree.append_new(tree.root().node_id(), "Node0");
            tree.append_new(node0_id, "Node0_0");
            tree.append_new(node0_id, "Node0_1");

            // Node 1: With attributes, with children.
            // Node 1-x: With attributes, without children.
            let node1_id = tree.append_new(tree.root().node_id(), "Node1");
            tree.append_attribute(node1_id, true);
            let node1_0_id = tree.append_new(node1_id, "Node1_0");
            tree.append_attribute(node1_0_id, 42i32);
            tree.append_attribute(node1_0_id, 1.234f64);
            let node1_1_id = tree.append_new(node1_id, "Node1_1");
            tree.append_attribute(node1_1_id, &[1u8, 2, 4, 8, 16] as &[_]);
            tree.append_attribute(node1_1_id, "Hello, world");

            tree
        };

        let tree_macro = tree_v7400! {
            Node0: {
                Node0_0: {},
                Node0_1: {},
            },
            Node1: [true] {
                Node1_0: (vec![42i32.into(), 1.234f64.into()]) {}
                Node1_1: [&[1u8, 2, 4, 8, 16][..], "Hello, world"] {}
            },
        };

        assert!(tree_manual.strict_eq(&tree_macro));
    }
}
