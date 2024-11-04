fn add(a:i32,b:i32) -> i32 {
    a + b
}
fn main(){
    let a = |a:i32,b:i32| a+b;
    println!("Result is {}",a(1,2));
    let b = |a,b,c,d| a + b + c +d ;
    println!("{}",b(1,2,3,4));
    // let a = add(2,3);
    // println! ("a:{}",a)
}

// fn main(){
//     let vec = vec![1,2,3,4,5];
//     let plus_one: Vec<_> = vec
//     .iter()
//     .map(|num|num+1)
//     .collect();
//     println!("plus_one is {:?}",plus_one);
//     // let mut plus_one = vec![];
//     // for num in vec {
//     //     plus_one.push(num*1);
//     //     println!("num is {}",num)
//     // }

// }

// fn main() {
//     println!("Hello, world!");
      
// }
// fn caps (input:&str) -> String {
//     input.to_uppercase()
// }

// #[cfg(test)]
// mod test {
//     use std::io::Result;

//     use crate::*;
//     #[test]
//     fn check(){
//         let result = caps("bui huu loc");
//         let expected = String::from("BUI HUU LOC");
//         assert_eq!(result,expected,"String should be all uppercase");
//     }
//     fn check2(){
//         let result = caps("bui huu loc");
//         let expected = String::from("BUI HUU LOC");
//         assert_eq!(result,expected,"String should be all uppercase");
//     }
// }
