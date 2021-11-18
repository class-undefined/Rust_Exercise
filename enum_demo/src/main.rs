#[derive(Debug)]
enum IpAddrKind{
    V4, V6
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String
}
fn main() {
    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("111")
    };
    println!("{:?}", home);
    println!("{:#?}", home);
}
