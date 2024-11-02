use std::collections::HashMap;

fn main() {
    // Tạo một HashMap với khóa kiểu String và giá trị kiểu u16
    let mut scores = HashMap::new();

    // Thêm dữ liệu vào HashMap
    scores.insert(String::from("MU"), 12);
    scores.insert(String::from("MC"), 20);

    // // In ra nội dung của HashMap
    // for (team, score) in &scores {
    //     println!("{}: {}", team, score);
    // }
    
}
