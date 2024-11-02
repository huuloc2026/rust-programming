
fn main() {
    println!("Helll World");
    // method 1
    let _v: Vec<u8> = Vec::new();
    // vec! call is marco
    let mut v1 = vec![1,2,3];
    v1.push(10);
    println!("V1: {:?}",v1);
    let first = v1[0];
    let first_other = v1.first();
    let method_get_first_other = v1.get(0);
    println!("First is {}",first);
    println!("First using FIRST() method is {:?}",first_other);
    println!("First using get() method is {:?}",method_get_first_other);
    for value in v1{
        println!("Item is {}",value)
    }
}


fn main() {
    let mut v = vec![1,2,3];
    
    for item in &v {
        println!("Item is {}",item)
    }
    for item in &mut v{
        *item+=1;
    }
}


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
    println!("colors = {:?}", colors);
}

