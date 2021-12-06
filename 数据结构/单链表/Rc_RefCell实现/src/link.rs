
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
type Node<T> = Option<Rc<RefCell<LinkNode<T>>>>;
struct LinkNode<T> {
    val: T,
    next: Option<Rc<RefCell<LinkNode<T>>>>
}
// #[debug()]
impl <T: Display>LinkNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val, next: None
        }
    }

    pub fn from(val: T, next: Node<T>) -> Self {
        Self {val, next}
    }

    pub fn to_ref_cell(self) -> RefCell<LinkNode<T>> {
        RefCell::new(self)
    }

    pub fn to_rc(self) -> Rc<RefCell<LinkNode<T>>> {
        Rc::new(self.to_ref_cell())
    }

    pub fn to_option(self) -> Node<T> {
        Some(self.to_rc())
    }

    pub fn set_next(&mut self, next: Node<T>) -> () {
        if next.is_none() {
            self.next = None;
            return
        }
        let next_node = next.as_ref().unwrap();
        self.next = Some(Rc::clone(next_node));
    }
}

pub struct Link<T> {
    root: Node<T>,
    pub size: usize
}

impl <T: Display>Link<T> {
    pub fn new() -> Self {
        Self {root: None, size: 0}
    }

    pub fn insert(&mut self, begin: usize, val: T) -> () {
        if begin > self.size {
            panic!("插入的位置begin: {} 不合法", begin);
        }
        let mut node = LinkNode::new(val);
        if begin == 0 {
            if self.root.is_none() {
                self.root = node.to_option();
            }else {
                node.set_next(Some(self.root.take().unwrap().clone()));
                self.root = node.to_option();
            }
            self.size += 1;
            return;
        }
        let mut curr = Rc::clone(self.root.as_ref().unwrap());
        for _i in 0..(begin - 1) {
            let _node = Rc::clone(&curr);
            curr = Rc::clone(&_node.as_ref().borrow_mut().next.as_ref().unwrap());
        }
        let post = if let Some(post_node) = &curr.as_ref().borrow_mut().next {
            Some(Rc::clone(post_node))
        } else {
            None
        };
        node.set_next(post);
        curr.as_ref().borrow_mut().set_next(node.to_option());
        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) -> () {
        if index >= self.size {
            panic!("删除的位置index: {} 不合法", index);
        }
        if self.root.is_none() {
            panic!("容器内无元素删除!");
        }
        /* 处理当删除元素是头节点的情况 */
        if index == 0 {
            let node_option = self.root.take();
            let node = node_option.unwrap();
            let next_node_option = &node.as_ref().borrow().next;
            if next_node_option.is_none() {
                self.root = None;
                self.size -= 1;
                return ;
            }
            let next_node = next_node_option.as_ref().unwrap();
            self.root = Some(Rc::clone(next_node));
            self.size -= 1;
            return ;
            /* 处理删除元素非头节点的情况 */
        } else {
            let mut curr_option = Rc::clone(self.root.as_ref().unwrap());
            for _i in 0..index - 1 {
                let next = Rc::clone(curr_option.borrow().next.as_ref().unwrap());
                curr_option = next;
            }
            let mut curr_node = curr_option.borrow_mut();
            let next_node_option = {
                if curr_node.next.is_none() {
                    None
                } else {
                    let next = curr_node.next.as_ref().unwrap();
                    Some(Rc::clone(next))
                }
            };
            curr_node.next = next_node_option;
            self.size -= 1;
        }


    }

    pub fn show(&self) -> () {
        let mut curr = Rc::clone(self.root.as_ref().unwrap());
        for _i in 0..self.size {
            // println!("i = {}", _i);
            let node = Rc::clone(&curr);
            let _node = Rc::clone(&curr);
            print!("{} ", node.as_ref().borrow().val);
            if node.as_ref().borrow().next.is_some() {
                curr = Rc::clone(&_node.as_ref().borrow_mut().next.as_ref().unwrap());
            }
        }
    }
}