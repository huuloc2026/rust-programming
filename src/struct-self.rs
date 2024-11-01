struct HINHCHUNHAT{
    dai: u32,
    rong: u32,
}
impl HINHCHUNHAT {
    fn dien_tich(&self) -> u32 {
        self.dai  * self.rong
    }
}
fn main() {
    let kichthuoc = &HINHCHUNHAT{dai:30,rong:50};
    println!("Dien tich is {}",kichthuoc.dien_tich());
}


