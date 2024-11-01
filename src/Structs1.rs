#[derive(Debug)]
struct Member {
    username: String,
    email: String,
    age: u64,
    active: bool,
}
fn main() {
    let mut member_1 = Member {
        username: String::from("username@gmail.com"),
        email: String::from("value@gmail.com"),
        age: 28,
        active: true,
    };
    let name = member_1.username;
    println!(">>>>>>>>name is {}", name);
    member_1.username = String::from("Huu Loc");
    println!(">>>>>>>>name changed is {}", name);

    let new_member = create_member(String::from("HuuLocFromFunc"), String::from("EmailFromFunc"), 25);
    println!(">>>>>>>>>>>new Member from func is {:?}",new_member);
    let new_member2 = Member {
        username: String::from("Hello new Member 2"),
        ..new_member
    };
    println!(">>>>>>>>>>>print newMember2 is {:?}",new_member2);
}

fn create_member(username: String, email: String, age: u64) -> Member {
    Member {
        username: username,
        email: email,
        age: age,
        active: true,
    }
}
