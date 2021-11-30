use std::{cell::RefCell, rc::Rc};


pub type Node<T> = Option<Rc<RefCell<TreeNode<T>>>>;


pub struct TreeNode<T> {
    val: T,
    left: Node<T>,
    right: Node<T>
}

impl TreeNode {
    
}
