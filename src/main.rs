
fn main() {
    let colors = vec!["Red", "Yellow", "Green"];
    
    // sử dụng into_iter()
    for color in colors.into_iter() {
        // mỗi phần tử sẽ drop sau khi kết thúc phạm 
        // vi duyệt
        println!("{}", color);
    }
    // hết phạm vi duyệt
    
    // Lỗi 
    // collection bị xoá 
    println!("colors = {:?}", color);
}

