use std::borrow::Borrow;
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
            curr = Rc::clone(&_node.borrow_mut().next.as_ref().unwrap());
        }
        let post = if let Some(post_node) = &curr.borrow_mut().next {
            Some(Rc::clone(post_node))
        } else {
            None
        };
        node.set_next(post);
        curr.borrow_mut().set_next(node.to_option());
        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) -> () {
        if index > self.size {
            panic!("删除的位置index: {} 不合法", index);
        }


    }

    pub fn show(&self) -> () {
        let mut curr = Rc::clone(self.root.as_ref().unwrap());
        for _i in 0..self.size {
            // println!("i = {}", _i);
            let node = Rc::clone(&curr);
            let _node = Rc::clone(&curr);
            print!("{} ", node.borrow_mut().val);
            if node.borrow_mut().next.is_some() {
                curr = Rc::clone(&_node.borrow_mut().next.as_ref().unwrap());
            }
        }
    }


    
}