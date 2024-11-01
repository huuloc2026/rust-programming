
fn main(){
    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    let sum = x + y.unwrap_or(1);
    println!("sum = {}",sum)
}
