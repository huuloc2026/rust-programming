fn main(){
    let alice = get_user("");
    match alice {
        Some(value) => println!("Get user is {:?}",value),
        None => println!("No have value")
        
    }
    
}   
fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}
