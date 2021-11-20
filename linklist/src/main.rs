#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
  
}
impl Clone for ListNode {
    fn clone(&self) -> Self{
        ListNode {
            val: self.val.clone(),
            next: self.next.clone()
        }
    }
}
fn create_link_list<'n>(nums: Vec<i32>) -> Option<Box<ListNode>> {
    if nums.len() == 1 {
        return Some(Box::new(ListNode::new(nums[0])));
    }
    let mut node = Some(Box::new(ListNode::new(nums[0])));
    let ans = node.clone();
    for i in nums.iter().skip(0) {
        if let Some(p) = &mut node {
            let cur:& 'n Option<Box<ListNode>> = &Some(Box::new(ListNode::new(*i)));
            p.next = cur;
            if let Some(_p) = &p.next {
                p = _p;
            }
        };

    }
    ans
}
fn main() {
    let mut a = ListNode::new(1);
    let b = ListNode::new(2);
    let c = ListNode::new(3);
    a.next = Some(Box::new(b));
    if let Some(n) = a.next {
        println!("{}", (*n).val)
    };
}
