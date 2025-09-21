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

// fn main (){
//     let maybe: Option<i32> = Some(7);


//   if  let Some(x) = maybe {
//     println! ("got: {}", x);
 

//     // matches! macro (boolean test)
//     let is_some = matches!(maybe, Some(_));
//     println!("is_some = {}", is_some);


//       // while let: iterate until pattern fails
//     let mut stack = vec![1, 2, 3];
//     while let Some(top) = stack.pop() {
//         println!("popped {}", top);
//   }
// }
// }



// #[derive(Debug)]


// enum TrafficLight { Red, Yellow, Green }
// impl TrafficLight {
//     fn time_to_wait(&self)-> u32{
//         match self {
//             TrafficLight::Red => 60,
//             TrafficLight::Yellow => 5,
//             TrafficLight::Green => 0,
//         }
//     }
// }


// fn main() {
//     let t = TrafficLight::Red;
//     println!("Wait {}s", t.time_to_wait());




#[repr(u16)]
enum HttpStatus {
    ok = 200,
    NotFound = 404,
    InternalServerError = 500,

}

fn main (){
    let s = HttpStatus::NotFound;
    let code: u16 = s as u16;
    println!("HTTP Status: {}", code);
}