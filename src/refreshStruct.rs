#[derive(Debug)]
struct Student{
    name:String,
    age:u8,
    class:String,
}
fn main() {
    let alice = Student {
        name: String::from("Alice"),
        age: 20,
        class:String::from("Rust Bootcamp"),
    };
    println!("Infor of Alice is {:?}",alice)

}
#[derive(Debug)]
struct Student{
    _name:String,
    _age:u8,
    _class:String,
}
struct Macbook;
impl Macbook {
    pub fn get_name(&self) -> String {
        "M1".to_string()
    } 
}
fn main() {
    // let alice = Student {
    //     _name: String::from("Alice"),
    //     _age: 20,
    //     _class:String::from("Rust Bootcamp"),
    // };
    // println!("Infor of Alice is {:?}",alice);
    // println!("age of Alice is {}",alice._age);
    let m1 = Macbook{};
    println!("Name of Macbook is {}",m1.get_name());
}


