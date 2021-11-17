/* 栈中的拷贝，所有权不会丧失，因为是拷贝赋值 */
fn test1() {
    let a = 1;
    let b = a;
    println!("{}, {}", a, &a==&b); 
}
fn test2() {
    let mut a:Vec<i32> = Vec::new();
    a.push(1);
    let b = a;
    println!("{}", b[0]); // 若进行其他语言中的引用赋值，地址被新的变量获取，则会丧失所有权。
}

/* Error:地址被其他变量获取，所有权丧失 */
fn test3() {
    let _v:Vec<i32> = Vec::new();
    _v.push(1);

    /* change函数得到了传递过来的Vec实例的所有权 */
    let change = |v:Vec<i32>| -> () {
        return;
    };
    change(_v);
    println!("{}", _v[0]);
}

/* 与javascript的const不同，Rust若不指定mut，则无法更改容器内的数据，这里的数据修改权不仅限于栈空间，更像是变量的可写权限？ */
fn test4() {
    let mut _v:Vec<i32> = Vec::new();
    _v.push(1);

    /* change函数得到了传递过来的Vec实例的所有权 */
    let change = |v:& mut Vec<i32>| -> () {
        v.push(3);
        return;
    };
    change(&mut _v);
    println!("{}", _v[1]);
}

fn main() {
    test1();
    test2();
}
