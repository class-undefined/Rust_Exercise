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
fn main() {
    println!("Hello, world!");
}
