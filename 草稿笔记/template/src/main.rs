#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}
impl <T: Clone, U: Clone> Point<T, U> {
    fn mixup<V: Clone, W: Clone>(self: &Point<T, U>, other:&Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x.clone(),
            y: other.y.clone()
        }
    }
}
fn main() {
    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "hello", y: 'c'};
    let p3 = p1.mixup(&p2);
    println!("{:#?}", p3);
}
