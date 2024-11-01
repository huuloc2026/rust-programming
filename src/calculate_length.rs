fn main() {
    let s1 = String::from("Hello world");
    // let a = calculate_length(s1);
    let (s2,length) = calculate_length(s1);
    println!("Result is {} and length is {}",s2,length)
}
fn calculate_length(string_input: String) -> (String,usize) {
    let length_string = string_input.len();
    (string_input,length_string)
}