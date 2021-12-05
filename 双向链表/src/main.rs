mod link;
use link::Link;

use crate::link::LinkMethod;
fn main() {
    let mut list:Link<i32> = Link::new();
    let v = vec![1,2,3,4,5];
    for i in v.iter().enumerate() {
        list.insert(i.0, *i.1);
    }
    list.show();
}
