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


enum Parcel {
    Box,
    Envelope,
    Tube,
}

fn main (){
    let parcel = Parcel::Envelope;

    match parcel{
        Parcel::Box => println!("It's a box!"),
        Parcel::Envelope => println!("It's an envelope!"),
        Parcel::Tube => println!("It's a tube!"),
    }

}