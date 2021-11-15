use std::io; // 使用命名空间？
use rand::Rng;
// cargo换国内源：https://www.cnblogs.com/baby123/p/13260212.html
// issue: cargo build触发 Blocking waiting for file lock on package cache 
fn main() {
    println!("猜数字");
    println!("请输入猜测的数字：");
    let secret_number = rand::thread_rng().gen_range(1..101); // 左闭右开？
    println!("该数字是：{}", secret_number);
    let mut guess = String::new(); // 创建一个String对象
    io::stdin().read_line(&mut guess).expect("输入错误"); // 异常处理？
    println!("你输入的数字是：{}", guess); // formatter
    
}
