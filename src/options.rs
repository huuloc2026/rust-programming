
fn main(){
     //    let _number = Some(5);
    //    let _string = Some("A String");
    //    let _nonenumber: Option<i32> = None;
    let x = 5;
    let y = None;
    let sum = x + y.unwrap_or(2);
    println!("sum is {}",sum)
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("six is {:#?}",six);
    let none = plus_one(None);
    println!("None is = {:#?}",none);
}
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        Some(x) => Some(x+1),
        _ => None,
    }
}

fn main() {
    let _value = Some(4);
     match _value {
         Some(5) => println!("bang 5"),
         _ => println!("khac 5")
     }
 }
 