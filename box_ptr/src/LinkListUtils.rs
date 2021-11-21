use std::fmt::{Debug, Display};

pub type Node<T: Clone> = Box<LinkNode<T>>;

pub struct LinkNode<T: Clone> {
    pub val: T,
    pub next: Option<Node<T>>
}
impl <T: Clone + Debug + Display>LinkNode<T> {
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

    pub fn get_next<'a> (&'a self) -> &'a Option<Node<T>> {
        &self.next
    }

    pub fn get_next_next<'a> (&'a self) -> &'a Option<Node<T>> {
        let node = self.get_next();
        println!("{}", self.val);
        match node {
            None => return &None,
            Some(next) => return next.get_next()
        }
    }

}

pub struct LinkList<T: Clone + Display> {
    head: Option<Box<LinkNode<T>>>,
    len: u32
}
impl <T: Clone + Display + Debug>LinkList<T> {
    pub fn new() -> Self {
            LinkList {
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

    pub fn pop_back(&mut self) -> Option<Node<T>> {
        match self.head.as_mut() {
            None => None,
            Some(mut curr) => {
                while curr.next.is_some() && curr.next.as_ref().unwrap().next.is_some() {
                    curr = curr.next.as_mut().unwrap();
                }
                self.len -= 1;
                match curr.next {
                    Some(_) => Some(curr.next.take().unwrap()),
                    None => Some(self.head.take().unwrap()),
                }
            }
        }
    }

    pub fn push_from_vec(&mut self, vec: Vec<T>) -> () {
        for e in vec {
            self.push_back(e);
        }
    }

    pub fn show(&mut self) -> () {
        println!("{}", self.size());
        let mut cursor = self.head.as_mut().expect("msg");
        for _i in 0..self.len  {
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

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

}
