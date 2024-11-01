fn main(){
    let x = 2;
    let square = |i| -> i32 {
        i*i
    }(x);
    println!("{}", square);
}