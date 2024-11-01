struct HINHCHUNHAT{
    dai: u32,
    rong: u32,
}
fn main() {
    let kichthuoc = &HINHCHUNHAT{dai:30,rong:50};
    println!("Dien tich is {}",dien_tich(kichthuoc));
}

fn dien_tich(kichthuoc:&HINHCHUNHAT)-> u32{
    kichthuoc.dai * kichthuoc.rong
}
