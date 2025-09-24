// fn main() {
//     println!("Hello, world!");
// }

// fn main() {
//     let maybe_number: Option<i32> = Some(5);
//     let maybe_nothing: Option<i32> = None;

//     println!("Number: {:?}", maybe_number);
//     println!("Nothing: {:?}", maybe_nothing);
// }

fn main() {
    let gift: Option<&str> = Some("Toy Car");

    match gift {
        Some(item) => println!("You got a {}", item),
        None => println!("The box is empty"),
    }
}
