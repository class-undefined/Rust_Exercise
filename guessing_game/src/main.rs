use std::io; // 使用命名空间？
fn main() {
    println!("猜数字");
    println!("请输入猜测的数字");
    let mut guess = String::new(); // 创建一个String对象
    io::stdin().read_line(&mut guess).expect("输入错误"); // 异常处理？
    println!("你输入的数字是：{}", guess); // formatter
}
