pub mod LinkListUtils;
use LinkListUtils::LinkList;
fn main() {
    let mut link: LinkList<i32> = LinkList::new();
    link.push_from_vec(vec![1,2,3,4]);
    link.insert(0, 10);
    // let a = link.remove(1);
    link.pop_back();
    link.show();
}
