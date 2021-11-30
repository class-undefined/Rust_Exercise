pub mod tree;
use tree::Tree;
fn main() {
    let a: Vec<Option<i32>> = vec![Some(1),None,Some(4),Some(5),Some(6),Some(7),Some(8)];
    let tree = Tree::from(&a);
    tree.preorder_traversal();
    // println!("{}", r);
}
