trait Clicky {
    fn click(&self);

}
struct Keyboard;
impl Clicky for Keyboard {
    fn click(&self) {
        println!("KeyBoard Input")
    }    
}
struct Mouse;
impl Clicky for Mouse {
    fn click(&self) {
        println!("Mouse Click")
    }
    
}
fn main(){
    let x = Keyboard;
    x.click();
    let y = Mouse;
    y.click();

}