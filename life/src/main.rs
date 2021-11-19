fn test<'n>() -> &'n i32 {
    let a:&'n i32 = &1;
    a
}

fn main() {
    println!("{}", test());
}
