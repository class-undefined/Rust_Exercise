pub mod LinkListUtils;
use LinkListUtils::LinkList;
fn main() {
    let mut link: LinkList<i32> = LinkList::new();
    link.push_from_vec(vec![1,2,3,4]);
    link.push_back(5);
    link.pop_back();
    link.pop_back();
    link.show();
}
