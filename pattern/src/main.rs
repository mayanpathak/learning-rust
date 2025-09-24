// fn main(){
//     let number = 2;

//     match number {
//     1 => println!("One!"),
//     2 | 3 => println!("Two or three!"),
//     4..=10 => println!("Between four and ten!"),
//     _ => println!("Something else!"),
// }

// }


// fn main (){
//     let day = "Saturday";
//     match day {
//         "Monday" => println!("Start of the work week!"),
//         "Friday" => println!("Last day of the work week!"),
//         "Saturday" | "Sunday" => println!("It's the weekend!"),
//         _ => println!("Midweek days are so-so."),
//     }
// }

// fn main (){
//     let score = 97;

//     match score{
//         0..=59 => println!("Failing grade"),
//         60..=69 => println!("Passing grade"),
//         70..=79 => println!("Average grade"),
//         80..=89 => println!("Good grade"),
//         90..=100 => println!("Excellent grade"),
//         _ => println!("Invalid score"),
//     }
// }

// fn main (){
//     let age = 21;
//     match age {
//         0..=12=> println!("Child"),
//         13..=19 => println!("Teenager"),
//         20..=64 => println!("Adult"),
//         65..=120 => println!("Senior"),
//         n => println!("adult with age {n}", ),
//         }
// }



// fn main(){
//     let point = (3,7);

// match point {
//     (0,0) => println!("Origin"),
//     (x,0) => println!("On the x-axis at x = {}", x),
//     (0,y) => println!("On the y-axis at y = {}", y),
//     (x,y) => println!("Point is at ({}, {})", x, y),
//     }
// }


// enum Parcel {
//     Box,
//     Envelope,
//     Tube,
// }

// fn main (){
//     let parcel = Parcel::Envelope;

//     match parcel{
//         Parcel::Box => println!("It's a box!"),
//         Parcel::Envelope => println!("It's an envelope!"),
//         Parcel::Tube => println!("It's a tube!"),
//     }

// }

// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
// }


// fn main(){
//     let msg = Message::Move{x: 10, y:20};


//     match msg{
//         Message::Quit => println!("Quitting"),
//         Message::Move{x, y} => println!("Moving to ({}, {})", x, y),
//         Message::Write(text) => println!("Writing: {}", text),
//     }
// }

// enum Shape {
//     Rectangle { width: i32, height: i32 },
//     Circle(i32),
// }

// fn main() {
//     let shape = Shape::Rectangle { width: 10, height: 20 };

//     match shape {
//         Shape::Rectangle { width, height } => {
//             println!("Rectangle {width}x{height}");
//         }
//         Shape::Circle(r) => println!("Circle with radius {r}"),
//     }
// }


// fn main() {
//     let number = 7;

//     match number {
//         n if n % 2 == 0 => println!("{n} is even"),
//         n if n % 2 != 0 => println!("{n} is odd"),
//         _ => println!("Unknown"),
//     }
// }


// fn process_delivery(result: Result<Option<i32>, String>) {
//     match result {
//         Ok(Some(weight)) if weight > 10 => println!("Heavy parcel: {weight}kg 🚚"),
//         Ok(Some(weight)) => println!("Light parcel: {weight}kg 📦"),
//         Ok(None) => println!("Empty parcel ❌"),
//         Err(e) => println!("Delivery failed: {e} ⚠️"),
//     }
// }

// fn main() {
//     process_delivery(Ok(Some(12)));
//     process_delivery(Ok(Some(5)));
//     process_delivery(Ok(None));
//     process_delivery(Err("Address not found".into()));
// }


// enum Option<T> {
//     Some(T),
//     None,
// }



fn main (){
    println!("Hello, world!");
}