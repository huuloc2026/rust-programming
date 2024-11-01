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
