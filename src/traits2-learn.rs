struct Data {
    num1: i32,
    num2: i32,
    str1: String,
    option: Option<i32>,
}
struct Data2 {
    num1: i32,
    num2: i32,
    str1: String,
    option: Option<i32>,
}
impl Data {
    fn new() -> Self {
        Data { num1: 15, num2: 20, str1: "someStringNew".to_string(), option:None }
    }
}
trait Transform {
    fn revert(&self)-> String {
        String::from("No String...")
    }
}

impl Transform for Data {
    fn revert(&self)-> String {
        self.str1.chars().rev().collect()
    }
}
impl Transform for Data2 {
    
    
}

fn main(){
    let a = Data::new();
    let b = Data2{
        num1:1,
        num2:20,
        str1:String::from("String01"),
        option:Some(10),
    };
    println!("Reverse String is {}",a.revert());
    println!("impl String is {}",b.revert());

}

