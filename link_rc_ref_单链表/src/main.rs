pub mod link;
use link::Link;
fn main() {
    let mut link: Link<i32> = Link::new();
    link.insert(0, 1);
    link.insert(1, 2);
    link.insert(2, 3);
    link.remove(2);
    link.show();
    // println!("{}", &link.size);
}
