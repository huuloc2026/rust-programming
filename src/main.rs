trait Clicky {
    fn click(&self) -> String;

}
struct Keyboard;
impl Clicky for Keyboard {
    fn click(&self) -> String {
        "KeyBoard Input".to_owned()
    }   
}
struct Mouse;
impl Clicky for Mouse {
    fn click(&self) -> String {
        "Mouse Click".to_owned()
    }
}
fn main(){
    //let x = Keyboard;
    //let x: &dyn Clicky = &Keyboard;
    // let y = Mouse;
        // x.click();
        // y.click();
    let x : Box<dyn Clicky> = Box::new(Keyboard);
    let y : Box<dyn Clicky> = Box::new(Mouse);

    let clickers = vec![x,y];

    for i in clickers {
        println!("click is {}",i.click())
    }
}