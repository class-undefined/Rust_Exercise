mod tree;
use tree::Tree;

use crate::tree::TreeMethod;
fn main() {
    let mut tree: Tree<i32> = Tree::new();
    let nodes = [Some(1), None, Some(2), Some(3), Some(4), None, None, Some(5), None, Some(6), None, None];
    tree.create(&nodes);
    let depth = tree.get_depth();
    println!("depth:{} tree: {:?}", depth, tree);
}
