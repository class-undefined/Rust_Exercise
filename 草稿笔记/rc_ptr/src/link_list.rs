#[warn(unused_mut)]
use std::{borrow::{Borrow, BorrowMut}, rc::Rc};
/* 节点属性 */
pub type Node<T> = Option<Rc<LinkNode<T>>>;
#[derive(Debug)]
pub struct LinkNode<T: Clone>{
    val: T,
    next: Node<T>
}
#[derive(Debug)]
pub struct Link<T: Clone> {
    head: Node<T>
}

impl <T: Clone>LinkNode<T> {
    fn new(val:T) -> LinkNode<T> {
        Self {
            val,
            next: None
        }
    }
    fn set_next(&mut self, other: Node<T>) -> () {
        self.next = other
    }
    fn get_last<'a> (&'a mut self) -> &'a mut Self{
        if let Some(ref mut node) = self.next {
            return node.get_last();
        }
        self
    }
}

pub trait LinkInterface<T: Clone> {
    fn new() -> T;
    fn push(node: LinkNode<T>) ->();
    // fn pop() -> (); // Todo
    // fn iter() -> (); // Todo
}
impl <T: Clone + LinkInterface<T>> Link<T> {
    fn new(val: T) -> Self {
        Self {
            head: Some(Rc::new(LinkNode::new(val)))
        }
    }

    fn push(&mut self, node: Node<T>) ->() {
        /* 如果next不为None，则_node = _node.next */
        if let Some(_head) = &mut self.head {
            if let Some(_node) = node {
                let reference = _head.borrow_mut().get_last();
                (*reference).get_last().set_next(Some(_node));
            }
        }
        ()
    }
}