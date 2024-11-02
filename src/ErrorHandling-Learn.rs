use std::fs::File;

fn main() {
    // let data_result = File::open("data.txt");
    // // match de unwrap
    // let data_file: File = match data_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the data file: {:?}", error),
    // };
    // println!("Data file is {:?}", data_file);

    //--------------- phuc hoi loi khi khong co file ton tai
    let data_result = File::open("data.txt");

    let data_file: File = match data_result {
        Ok(file) => file,
        Err(_) => {
            match File::create("data2.txt") {
                Ok(new_file) => new_file,
                Err(error) => panic!("Problem creating a new data file: {:?}", error),
            }
        }
    };

    println!("Data file: {:?}", data_file);

}

fn main(){
    let alice = get_user("");
    //case 1 using match
    match alice {
        Some(value) => println!("Get user is {:?}",value),
        None => println!("No have value"),
        
    }
    //case 2 using unwrap.
    let bob_unwrap = get_user("Bob2").unwrap();
    println!("Bob value: {bob_unwrap}");
    let charlie_unexpect = get_user("").expect("====Please input your name, Charlie!!");
    println!("Bob value: {charlie_unexpect}");     

    
}   
fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}
