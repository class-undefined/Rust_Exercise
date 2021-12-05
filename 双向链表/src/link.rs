use std::{rc::Rc};
use core::cell::RefCell;
use core::fmt::Debug;
type Node<T> = Option<Rc<RefCell<LinkNode<T>>>>;
#[derive(Debug)]
pub struct LinkNode<T> where T: Debug + Clone {
    val: T,
    pre: Node<T>,
    next: Node<T>
}

impl <T>LinkNode<T> where T: Debug + Clone {
    fn new(val: T, pre: Node<T>, next: Node<T>) -> LinkNode<T> {
        Self {
            val, pre, next
        }
    }

    fn set_pre(&mut self, pre: Node<T>) -> () {
        self.pre = pre
    }

    fn set_next(&mut self, next: Node<T>) -> () {
        self.next = next;
    }

    fn get_pre_val(&self) -> Option<T> {
        if self.pre.is_none() {
            None
        } else {
            let n = self.pre.as_ref().unwrap().borrow();
            Some(n.val.clone())
        }
    }

    fn get_next_val(&self) -> Option<T> {
        if self.next.is_none() {
            None
        } else {
            let n = self.next.as_ref().unwrap().borrow();
            Some(n.val.clone())
        }
    }

    fn to_ref_cell(self) -> RefCell<LinkNode<T>> {
        RefCell::new(self)
    }

    fn to_rc(self) -> Rc<RefCell<LinkNode<T>>> {
        Rc::new(self.to_ref_cell())
    }


}

#[derive(Debug)]
pub struct Link<T> where T: Debug + Clone {
    head: Node<T>,
    size: usize
}

pub trait LinkMethod<T> {
    fn insert(&mut self, begin: usize, val: T) -> ();

    fn remove(&mut self, begin: usize) -> ();

    fn show(&self) -> ();
}

impl <T>Link<T> where T: Debug + Clone {
    pub fn new() -> Link<T> {
        Self {head: None, size: 0}
    }
}

impl <T>LinkMethod<T> for Link<T> where T: Debug + Clone {
    fn insert(&mut self, begin: usize, val: T) -> () {
        if begin == 0 {
            if self.head.is_none() {
                self.head = Some(LinkNode::new(val, None, None).to_rc());
                self.size += 1;
            }else {
                let old_head_option = self.head.take();
                let old_head = Rc::clone(&old_head_option.unwrap());
                /* 新头节点的next置为旧节点 */
                let new_node = LinkNode::new(val, None, Some(Rc::clone(&old_head)));
                let new_node = new_node.to_rc();
                /* 旧节点的pre置为新节点 */
                old_head.borrow_mut().set_pre(Some(Rc::clone(&new_node)));
                self.head = Some(new_node);
                self.size += 1;
            }
        } else {
            let mut curr_node = Rc::clone(&self.head.as_ref().unwrap());
            let _curr = Rc::clone(&curr_node);
            for _i in 0..(begin - 1) {
                let t = curr_node.clone();
                let i = {
                    if _i != 0 {
                        Rc::clone(t.borrow().next.as_ref().unwrap())
                    } else {
                        Rc::clone(curr_node.borrow().next.as_ref().unwrap())
                    }
                };
                curr_node  = i;
                /* debug begin */
                // let p = Rc::clone(&curr_node);
                // println!("for: {:?}", p.borrow().val);
                /* debug end */
            }
            /* == */
            let mut o = curr_node.borrow_mut();
            let post_node = {
                let post = o.next.as_ref();
                if post.is_none() {
                    None
                }else {
                    Some(Rc::clone(post.unwrap()))
                }
            };
            let new_node = LinkNode::new(val, Some(Rc::clone(&curr_node)), post_node.clone()).to_rc();
            /* == */
            
            o.set_next(Some(Rc::clone(&new_node)));
            if post_node.is_some() {
                post_node.unwrap().borrow_mut().set_pre(Some(Rc::clone(&new_node)));
            }
            self.size += 1;
        }
    }

    fn remove(&mut self, begin: usize) -> () {
        if self.size <= 0 {
            panic!("Link 中 元素已为0")
        }
        if begin == 0 {
            if self.head.is_none() {
                panic!("Link 中 head 为None")
            }
            let head = self.head.take();
            let head_rc = head.unwrap();
            let head_ref_cell = head_rc.borrow();
            let head_next_rc_ref = head_ref_cell.next.as_ref().unwrap();
            head_next_rc_ref.borrow_mut().pre = None; // 取消前驱节点
            self.head = Some(Rc::clone(head_next_rc_ref));
            self.size -= 1;
        }else {
            let mut curr_node = Rc::clone(self.head.as_ref().unwrap());
            for _ in 0..begin - 1 {
                let next_node = Rc::clone(curr_node.borrow().next.as_ref().unwrap());
                curr_node = next_node;
            }
            let curr_mut = Rc::clone(&curr_node);
            let curr_node_borrow = curr_node.borrow();
            let post_node_ref = curr_node_borrow.next.as_ref().unwrap();
            let post_node = Rc::clone(post_node_ref);
            curr_mut.borrow_mut().set_next(Some(Rc::clone(post_node_ref)));
            post_node.borrow_mut().set_pre(Some(Rc::clone(&curr_mut)));
            self.size -= 1;
        }
    }

    fn show(&self) -> () {
        let mut curr_node = Rc::clone(self.head.as_ref().unwrap());
        let mut has_next = curr_node.as_ref().borrow().next.is_some();
        while has_next {
            let o = curr_node.clone();
            let node = o.borrow();
            print!("{{ pre: {:?} curr: {:?} next: {:?} }}",node.get_pre_val(), node.val, node.get_next_val());
            let next = node.next.as_ref().unwrap().clone();
            curr_node = next;
            has_next = curr_node.as_ref().borrow().next.is_some();
            if has_next {
                print!(" -> ");
            }
        }
    }
}