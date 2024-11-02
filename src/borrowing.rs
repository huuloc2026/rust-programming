fn main() {
    let fruits = String::from("Hello world");
    print_fruit(fruits.clone());
    println!("> value copy is {}",fruits)
}
fn print_fruit(str:String){
    println!("value is {}",str)
}

// another way reference
fn main() {
    let fruits = String::from("Hello world");
    print_fruit(&fruits);
    //borrowing
    println!("> value copy is {}",fruits)
}
fn print_fruit(str:&String){
    println!("value is {}",str)
}

fn main() {
    let mut fruits = String::from("Hello world");
    print_fruit(&mut fruits);
    //borrowing
    println!("> value copy is {}",fruits)
    //mutable reference

}


fn print_fruit(str:&mut String){
    println!("value is {}",str);
    let _new_fruit = str.push_str(" - accepted by ownership!!");
}

fn main() {
    let mut fruits = String::from("Hello world");
    print_fruit(&mut fruits);
    //borrowing
    println!(">AFTER value copy is {}",fruits)
    //mutable reference

}


fn print_fruit(str:&mut String){
    println!(">>BEFORE value is {}",str);
    let _new_fruit = str.push_str(" - accepted by ownership!!");
}
