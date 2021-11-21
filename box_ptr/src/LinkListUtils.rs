use std::fmt::Display;

pub type Node<T: Clone> = Box<LinkNode<T>>;

pub struct LinkNode<T: Clone> {
    pub val: T,
    pub next: Option<Node<T>>
}
impl <T: Clone>LinkNode<T> {
    pub fn new(val:T) -> Self {
        LinkNode {
            val,
            next: None
        }
    }

    pub fn from(val:T, next: Option<Node<T>>) -> Self{
        LinkNode {
            val, next
        }
    }

}

pub struct LinkList<T: Clone + Display> {
    head: Option<Box<LinkNode<T>>>,
    len: u32
}
impl <T: Clone + Display>LinkList<T> {
    pub fn new() -> LinkList<T> {
        Self {
            head: None,
            len: 0
        }
    }
    pub fn push_back(&mut self, e: T) -> () {
        let node = LinkNode::new(e);
        if self.head.is_none() {
            self.head = Some(Box::new(node));
            self.len += 1;
            return;
        }
        let mut cursor = self.head.as_mut().expect("self.head.as_mut() error");
        /* 如果next不为none */
        while cursor.next.is_some() {
            let node_next = cursor.next.as_mut().expect("cursor.next.as_mut() error");
            cursor = node_next;
        }
        cursor.next = Some(Box::new(node));
        self.len += 1;
    }

    pub fn push_from_vec(&mut self, vec: Vec<T>) -> () {
        for e in vec {
            self.push_back(e);
        }
    }

    pub fn show(&mut self) -> () {
        let mut cursor = self.head.as_mut().expect("msg");
        for i in 0..self.len  {
            print!(" {} ", cursor.val);
            if cursor.next.is_some() {
                print!("->");
                cursor = cursor.next.as_mut().expect("无法到达next");
            }
        }
    }

    pub fn size(&mut self) -> u32 {
        self.len
    }

}
pub trait LinkTrait<T> {

    // fn insert(index: i32) -> ();

    fn push_from_vec(&mut self, v: Vec<T>) -> ();

    fn push_back(&mut self, e: T) -> ();

    // fn pop_back() -> ();

    fn show(&mut self) -> ();

    fn size(&mut self) -> u32;
}

impl <T: Clone + Display> LinkTrait<T> for LinkList<T> {
    fn push_back(&mut self, e: T) -> () {
        let node = LinkNode::new(e);
        if self.head.is_none() {
            self.head = Some(Box::new(node));
            self.len += 1;
            return;
        }
        let mut cursor = self.head.as_mut().expect("self.head.as_mut() error");
        /* 如果next不为none */
        while cursor.next.is_some() {
            let node_next = cursor.next.as_mut().expect("cursor.next.as_mut() error");
            cursor = node_next;
        }
        cursor.next = Some(Box::new(node));
        self.len += 1;
    }

    fn push_from_vec(&mut self, vec: Vec<T>) -> () {
        for e in vec {
            self.push_back(e);
        }
    }

    fn show(&mut self) -> () {
        let mut cursor = self.head.as_mut().expect("msg");
        while cursor.next.is_some() {
            print!("{} ", cursor.val);
            cursor = cursor.next.as_mut().expect("msg");
        }
    }

    fn size(&mut self) -> u32 {
        self.len
    }

}