fn test<'n>() -> &'n i32 {
    let a:&'n i32 = &1;
    a
}

fn getVec() -> Vec<i32> {
    let r:Vec<i32> = Vec::new();
    r
}

fn iterator() {
    let v: Vec<i32> = vec![1,2,3,4,5,6];
    let m:Vec<_> = v.iter().filter(|n| {
        n > &&3
    }).collect();
    println!("{} {}", v.len(), m.len());
}

fn main() {
    let x: Vec<i32> = Vec::new();
    let f = |a:Vec<i32>| &a.len() == &x.len();
    iterator();
    println!("{}", x.len());
}
