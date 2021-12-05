fn main() {
    let one: Option<i32> = Some(1);
    let two: Option<i32> = Some(2);
    match one {
        Some(v) => {
            println!("{}", v);
        },
        None => {}
    }
    println!("{:?} {:?}", one, two.as_ref());
    /* 对枚举的值进行匹配*/
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

}
