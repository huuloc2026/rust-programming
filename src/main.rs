fn main() {
    fn _x(){
        let _a = "hello";
        let _b = 100;
        _y();
    }
    fn _y(){
        let mut _a = String::from("World");
    _a.push_str("!");
    }
}
