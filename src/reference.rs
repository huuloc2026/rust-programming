fn main() {
    let mut s1: String = String::from("Hello");
    let _len = calculate_length(&mut s1);
    println!("Length is {}", _len);
}
fn calculate_length(string_input: &mut String) -> usize {
    string_input.push_str("World");
    let length_string: usize = string_input.len();
    length_string
}
