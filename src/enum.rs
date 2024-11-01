#[derive(Debug)]
enum IPAddressKind{
    V4,
    V6
}
#[derive(Debug)]
struct _IpAdress {
    kind: IPAddressKind,
    address: String,
}
fn main() {
    let four = IPAddressKind::V4;
    let six = IPAddressKind::V6;
    let localhost = _IpAdress{
        kind: IPAddressKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!(">>>>>address localhost is {:#?}",localhost);
}

#[derive(Debug)]
enum IPAddressKind{
    V4,
    V6
}
#[derive(Debug)]
struct _IpAdress {
    kind: IPAddressKind,
    address: String,
}
impl _IpAdress {
    fn some_function(){
        println!("Hello from Impl _IpAddress")
    }
}
fn main() {
    let four = IPAddressKind::V4;
    let six = IPAddressKind::V6;
    let localhost = _IpAdress{
        kind: IPAddressKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!(">>>>>address localhost is {:#?}",localhost);
    
}

