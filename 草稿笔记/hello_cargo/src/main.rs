fn heap() ->Box<i32> {
    let a = Box::new(1);
    a
}
fn main() {
    println!("Hello, world!");
    let a = vec![1];
    let b = a.clone();
    let a = heap();
    println!("{}", a);
}
