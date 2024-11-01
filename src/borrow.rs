fn main2() {
    let _s1 = gives_onwership();
    let s2 = String::from("Hello from s2");
    let _s3 = takes_and_return(s2);
    println!("{}",_s1);
    println!("{}",_s3);
}

fn gives_onwership() -> String {
    let some_string = String::from("Hello from s1");
    some_string
}
fn takes_and_return(_string:String) -> String {
    _string
}