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