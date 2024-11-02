fn main(){
    let vios = Car{category:"Sedan".to_string()};
    let _speed_vios =  vios.speed();
    let kawasaki = Motobike {category: "High-Speed Motor".to_string()};
    let _speed_kawa = kawasaki.speed();
}

pub struct Car{
    category: String,
}
pub struct Motobike {
    category: String,
}

// impl Car {
//     fn get_category(&self){
//         println!("Category:{}",self.category);
//     }
// }

pub trait Vehicle { 
    fn get_category(&self) -> String;
    fn speed(&self) -> u32;
}
// implement trait
impl Vehicle for Car {
    fn get_category(&self) -> String {
        self.category.clone()
    }
    fn speed(&self) -> u32 {
        100
    }
}
impl Vehicle for Motobike {
    fn get_category(&self) -> String {
        self.category.clone()
    }

    fn speed(&self) -> u32 {
        60
    }
}