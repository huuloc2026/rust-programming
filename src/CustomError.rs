fn main(){
    let file_path = "sample.txt";

   // Sử dụng match 
   match read_file_contents(file_path) {
       Ok(contents) => println!("File contents: {}", contents),
       Err(err) => eprintln!("Error reading file: {}", err),
   }
}
use std::fs::File;
use std::io::Read;

fn read_file_contents(path: &str) -> Result<String, std::io::Error> {
   // Sử dụng toán tử ? để unwrap lấy giá trị 
   // vì hàm open trả về Result
   let mut file = File::open(path)?;
   let mut contents = String::new();

   // Sử dụng toán tử ? để unwrap lấy giá trị
   file.read_to_string(&mut contents)?;

   Ok(contents) 
}

fn main(){
    let file_path = "sample.txt";

   // Sử dụng match 
   match read_file_contents(file_path) {
       Ok(contents) => println!("File contents: {}", contents),
       Err(err) => eprintln!("Error reading file: {}", err),
   }
}
#[derive(Debug)]
enum CustomError {
   FileOpenError,
   FileReadError

}
use std::fs::File;

use std::io::Read;
fn read_file_contents(path: &str) -> Result<String, CustomError> {
   // Sử dụng toán tử ? để unwrap lấy giá trị 
   // vì hàm open trả về Result
   let mut file = File::open(path).map_err(|_| CustomError::FileOpenError?);
   let mut contents = String::new();

   // Sử dụng toán tử ? để unwrap lấy giá trị
   file.read_to_string(&mut contents).map_err(|_| CustomError::FileReadError)?;

   Ok(contents) 
}