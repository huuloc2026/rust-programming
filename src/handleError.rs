// Authentication
// Authentization

struct Employee {
    position: Position,
    status: Status,
}
enum Position {
    CEO,
    CFO,
    IT,
    Manager,
}
enum Status{
    Active,
    Denied,
}

fn try_access(employee_params: &Employee) -> Result<(),String> {
    match employee_params.status {
        Status::Denied => return Err("Access Denied".to_owned()),
        _ => (),
    };
    match employee_params.position {
        Position::CEO => Ok(()),
        Position::CFO => Ok(()),
        Position::IT => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Invalid Position".to_owned())
    }
}
fn main(){
    let manager = Employee {
        position: Position::Manager,
        status: Status::Active,
    };
    let access_test = try_access(&manager);
    if access_test.is_ok() {
        println!("success access")
    }
}

