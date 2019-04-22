//! Tree construction macro test.
#![cfg(all(feature = "tree", feature = "writer"))]

use fbxcel::{tree::v7400::Tree, tree_v7400};

#[test]
fn compare_empties() {
    let tree1 = tree_v7400! {};
    let tree2 = tree_v7400! {};
    assert!(tree1.strict_eq(&tree2));
}

#[test]
fn compare_trees() {
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
            Node2: [true, 42i16, 42i32, 42i64, 1.414f32, 3.14f64] {
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
        tree.append_attribute(node1_0_id, 3.14f64);
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
            Node1_0: (vec![42i32.into(), 3.14f64.into()]) {}
            Node1_1: [&[1u8, 2, 4, 8, 16][..], "Hello, world"] {}
        },
    };

    assert!(tree_manual.strict_eq(&tree_macro));
}
