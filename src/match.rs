// Match Options
fn main(){
    decimals(Coin::Solana);
    decimals(Coin::Bitcoin((Balance::Shark)));
}
#[derive(Debug)]
enum Coin { 
    Solana,
    Ethereum,
    Near,
    Bitcoin(Balance),
}
#[derive(Debug)]
enum Balance {
    Small,
    Intermediate,
    Fish,
    Shark
}
fn decimals(coin:Coin) -> u8 {
    match coin {
        Coin::Solana => {
            println!(">>>>Match Solana");
            1
        },
        Coin::Ethereum => 10,
        Coin::Near => 99,
        Coin::Bitcoin(bala) => {
            println!(">>>>i am {:#?}",bala);
            11
        },
        
    }

}


fn main(){
    let five = Some(5);
    let six = plus_one(five);
    println!("six = {:#?}",six);
    let none = plus_one(None);
    println!("None = {:#?}",none);
}
fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
       Some(x) => Some(x+1),
       _ => None,
    }
}