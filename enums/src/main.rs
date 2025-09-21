// #[derive (Debug)]

// enum Direction {North, East, South, West}

// fn main(){
//     let dir = Direction::East;
    

//     match dir {
//         Direction::North => println!("Going North!"),
//         Direction::East => println!("Going East!"),
//         Direction::South => println!("Going South!"),
//         Direction::West => println!("Going West!"),
//     }
// }


// enum Message{
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColour(u8, u8, u8),
// }


// fn Handle (msg: Message){
//     match msg {
//         Message::Quit => println!("quit"),
//         Message::Move {x,y} => println!("Move to ({}, {})", x, y),
//          Message::Write(s) => println!("Write: {}", s),
//         Message::ChangeColour(r, g, b) => println!("Color: #{:02x}{:02x}{:02x}", r, g, b)
//     }
// }
// fn main (){
//     let answer = Message::Move {x: 10, y: 20};
//     Handle(answer);
// }



