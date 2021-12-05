pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}
impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
#[derive(Debug)]
struct A {
    val: i32
}
fn main() {
    let a = A {val: 1};
    let ref b = a;

    println!("Hello, world!{:?}", *b);
    println!("Hello, world!{:?}", *b);
}
