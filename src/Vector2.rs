fn main(){
    #[derive(Debug)]
    pub enum SheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SheetCell::Int(5),
        SheetCell::Float(10.12),
        SheetCell::Text(String::from("Hello World")),
    ];
    match &row[2] {
        &SheetCell::Float(i) => println!("value is {}",i),
        _ => println!("This is not float")
        
    }
    


}