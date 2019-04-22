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
