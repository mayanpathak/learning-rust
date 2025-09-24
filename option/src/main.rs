// fn main() {
//     println!("Hello, world!");
// }

// fn main() {
//     let maybe_number: Option<i32> = Some(5);
//     let maybe_nothing: Option<i32> = None;

//     println!("Number: {:?}", maybe_number);
//     println!("Nothing: {:?}", maybe_nothing);
// }

// fn main() {
//     let gift: Option<&str> = Some("Toy Car");

//     match gift {
//         Some(item) => println!("You got a {}", item),
//         None => println!("The box is empty"),
//     }
// }


// fn main() {
//     let value = Some(10);
//     println!("Value is: {}", value.unwrap()); // prints 10

//     let nothing: Option<i32> = None;
//     // println!("{}", nothing.unwrap()); // 💥 panic!
// }


// fn main() {
//     let value = None;
//     println!("Got: {}", value.unwrap_or(42)); // default to 42
// }



// fn devide (a: i34, b:32)-> Result<i32, &'static str> {
//     if b == 0{
//         Err("cannot devide by zero")

//     } else {
//         Ok(a/b)
//     }
// }


// fn main (){
//     match devide (10, 2){
//         Ok (Result)=> println!("Result: {}", Result),
//         Err (e) => println!("Error: {}", e),
//     }
// }


// fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
//     if b == 0 {
//         Err("Cannot divide by zero")
//     } else {
//         Ok(a / b)
//     }
// }

// fn main() {
//     match divide(10, 2) {
//         Ok(result) => println!("Result: {}", result),
//         Err(e) => println!("Error: {}", e),
//     }
// }



