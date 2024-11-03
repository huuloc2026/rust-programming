fn main() {
    let num1 = 10;
    let num2 = 30;
    let result = get_ref(&num1, &num2);
    println!("Result is {}", result);
}
fn get_ref<'a, 'b>(param_1: &'a i32, param_2: &'b i32) -> &'b i32 {
    println!("param2 is {}", param_2);
    param_2
}
  