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

