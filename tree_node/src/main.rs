pub mod tree;
use tree::Tree;
fn main() {
    let a: Vec<Option<i32>> = vec![Some(1),Some(2),Some(3),Some(3)];
    let tree = Tree::from(&a);
    let r = tree.root.val;
    println!("{}", r);
}
