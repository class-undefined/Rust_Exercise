#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}
impl <T, U> Point<T, U> {
    fn mixup<V, W>(self: Point<T, U>, other:Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}
fn main() {
    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("{:#?}", p3);
}
