use std::{thread, time::Duration};

fn msg_hello() -> &'static str {
    std::thread::sleep(Duration::from_secs(1));
    "Hello,"
}

fn msg_myname() -> &'static str {
    std::thread::sleep(Duration::from_secs(2));
    "my Name is"
}

fn msg_name() -> &'static str {
    std::thread::sleep(Duration::from_secs(3));
    "Bui Huu Loc"
}

fn msg_excited() -> &'static str {
    std::thread::sleep(Duration::from_secs(1));
    "!!!"
}
fn main(){
    let msg_one = thread::spawn(move || msg_hello());
    let msg_two = thread::spawn(move || msg_myname());
    let msg_three = thread::spawn(move || msg_name());
    let msg_four = thread::spawn(move || msg_excited());
    let msg_one = msg_one.join().expect("msg error");
    let msg_two = msg_two.join().expect("msg error");
    let msg_three = msg_three.join().expect("msg error");
    let msg_four = msg_four.join().expect("msg error");
    println!("{}{}{}{}",msg_one,msg_two,msg_three,msg_four);
    
}