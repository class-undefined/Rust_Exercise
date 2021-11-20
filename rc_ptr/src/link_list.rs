use std::rc::Rc;
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

    fn push(&self, node: Node<T>) ->() {
        let mut _node = &self.head;
        /* 如果next不为None，则_node = _node.next */
        loop {
            if let Some(n) = _node {
                _node = &Some(*n); // 不能这么做，块末尾会将n清除...
                continue;
            }
            break;
        }
        _node.is_none();
        if let Some(n) = &mut _node {
            n.next = node;
        };
        ()
    }
}