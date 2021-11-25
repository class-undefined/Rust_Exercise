pub mod tree;
use tree::Tree;
fn main() {
    let a: Vec<Option<i32>> = vec![Some(1),None,Some(4),Some(5)];
    let tree = Tree::from(&a);
    tree.preorder_traversal();
    // println!("{}", r);
}
