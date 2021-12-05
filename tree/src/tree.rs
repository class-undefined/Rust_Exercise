pub type Node<T: Copy> = Option<Box<TreeNode<T>>>;

#[derive(Debug)]
pub struct TreeNode<T: Copy> {
    val: T,
    left: Node<T>,
    right: Node<T>
}

#[derive(Debug)]
pub struct Tree<T: Copy> {
    root: Node<T>,
    depth: usize
}

pub trait TreeNodeMethod<T: Copy> {
    fn new(val: T, left: Node<T>, right: Node<T>) -> TreeNode<T> {
        TreeNode {
            val, left, right
        }
    }

}

pub trait TreeMethod<T: Copy> {
    fn new() -> Self;

    fn create(&mut self, nodes: &[Option<T>]) -> ();
}

impl <T: Copy>TreeNodeMethod<T> for TreeNode<T> {
}

impl <T: Copy>TreeMethod<T> for Tree<T> {
    fn new() -> Tree<T> {
        Tree {
            root: None, depth: 0
        }
    }

    fn create(&mut self, nodes: &[Option<T>]) -> () {
        fn _build<T: Copy>(_nodes: &[Option<T>], _index: &mut usize) -> Node<T> {
            if _index.clone() >= _nodes.len() {
                *_index += 1;
                return None;
            }
            let val = &_nodes[_index.clone()];
            if val.is_none() {
                *_index += 1;
                None
            } else {
                let mut node = TreeNode::new(val.unwrap(), None, None);
                *_index += 1;
                node.left = _build(_nodes, _index);
                node.right = _build(_nodes, _index);
                Some(Box::new(node))
            }
            

        };
        self.root = _build(nodes, &mut 0);
    }
}