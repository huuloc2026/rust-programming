use std::collections::HashMap;
fn main() {
    let mut user = HashMap::new();
    user.insert("username", "Huuloc");
    user.insert("nickname", "Jake");
    println!("User: {:?}",user);
    let username = user.get("username");
    println!("username is {:?}",username);
    for item in user.iter() {
        println!("Key is {} - value is {}",item.0,item.1)
    }
    let len = user.len();
    println!("Len of User is {}",len);
    if user.contains_key("username"){
        println!("Yes")
    }
    else {
        println!("No")
    }

}

