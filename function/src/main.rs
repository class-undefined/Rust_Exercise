use std::ptr::null;

/* 匿名函数，用竖线表示函数签名 */
fn test1() {
    let a = || -> i32 {
        return 2;
    };
    let b = a();
    println!("{}", b);
}
/* 地址被其他变量获取，所有权丧失 */
fn test2() {
    let _v:Vec<i32> = Vec::new();
    _v.push(1);

    /* change函数得到了传递过来的Vec实例的所有权 */
    let change = |v:Vec<i32>| -> () {
        return;
    };
    change(_v);
    println!("{}", _v[0]);
}
fn main() {
   test1();
   test2();
}
