fn main(){
    #[derive(Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let direction = Direction::Left;
    println!("Direct is {:#?}",direction);
    match direction {
        Direction::Up => println!("Robot go UP"),
        Direction::Down => println!("Robot go Down"),
        Direction::Left => println!("Robot go Left"),
        Direction::Right => println!("Robot go Right"),
    }
}

fn main() {
    let mood_now = Mood::_Angry;
    let level= match_mood(&mood_now);
    println!("Level mood now is {level}");
}
#[derive(Debug)]
enum Mood {
    _Happy,
    _Sleepy,
    _NotBad,
    _Angry,
}
fn match_mood(mood: &Mood) -> i32 {
    let happiness_level: i32 = match mood {
        Mood::_Happy => 10,
        Mood::_Sleepy => 6,
        Mood::_NotBad => 7,
        Mood::_Angry => 2,
    };
    happiness_level
}
