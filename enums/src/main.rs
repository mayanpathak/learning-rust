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




// #[repr(u16)]  // control how the enum is laid out for FFI or stable discriminants
// enum HttpStatus {
//     ok = 200,
//     NotFound = 404,
//     InternalServerError = 500,

// }

// fn main (){
//     let s = HttpStatus::NotFound;
//     let code: u16 = s as u16;
//     println!("HTTP Status: {}", code);
// }

//When to use: #[repr(...)] is useful for FFI or when you need numeric discriminants. Without repr, the compiler chooses representation.



// use std::boxed::Box;

// enum List{
//     Cons(i32, Box<List>),
//     Nil,
// }

// impl List{
//     fn sum(&self)-> i32 {
//         match self {
//             List::Cons(head, tail) => head + tail.sum(),
//             List::Nil => 0,
//         }
//     }
// }


// fn main (){
//     let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3,Box::new(List::Nil))))));
//     println!("Sum: {}", list.sum());//6
// }
// //Key point: recursive variants must be Box, Rc, or another indirection type so the enum has finite size. Use Rc when sharing is needed.



// enum MaybeRef<'a, T> {
//     Borrowed(&'a T),
//     Owned(T),
// }

// impl <'a, T> MaybeRef<'a, T>{
//     fn as_ref(&self)-> &T{
//         match self{
//             MaybeRef::Borrowed(r)=> r,
//             MaybeRef::Owned(t)=> t,
//         }
//     }
// }

// fn main (){
//     let s = String::from("hello");
//     let borrowed = MaybeRef::Borrowed(&s);
//     let owned = MaybeRef::Owned(String::from("world"));

//     println!("borrowed: {}", borrowed.as_ref());
//     println!("owned: {}", owned.as_ref());  
// }

// enum Token { Number(i32), Ident(String), Eof }

// fn describe(tok: Token) {
//     match tok {
//         Token::Number(n @ 1..=10) => println!("Small number: {}", n), // @ binds the matched value
//         Token::Number(n) if n % 2 == 0 => println!("Even number: {}", n), // guard
//         Token::Ident(ref s) => println!("Identifier: {}", s), // ref to avoid taking ownership
//         Token::Eof => println!("End"),
//         _ => println!("Other"),
//     }
// }


// // main.rs
// #[derive(Debug)]
// enum Token { Number(i32), Ident(String), Eof }

// // 1) Original — takes ownership of the Token
// fn describe_owned(tok: Token) {
//     match tok {
//         Token::Number(n @ 1..=10) => println!("Small number: {}", n),
//         Token::Number(n) if n % 2 == 0 => println!("Even number: {}", n),
//         Token::Ident(ref s) => println!("Identifier: {}", s),
//         Token::Eof => println!("End"),
//         _ => println!("Other"),
//     }
// }

// // 2) Reordered version (even-first)
// fn describe_reordered(tok: Token) {
//     match tok {
//         Token::Number(n) if n % 2 == 0 => println!("Even number: {}", n),
//         Token::Number(n @ 1..=10) => println!("Small number: {}", n),
//         Token::Ident(ref s) => println!("Identifier: {}", s),
//         Token::Eof => println!("End"),
//         _ => println!("Other"),
//     }
// }

// // 3) Borrowed version: accept &Token so we don't move the Token
// fn describe_borrowed(tok: &Token) {
//     match tok {
//         &Token::Number(n @ 1..=10) => println!("(borrowed) Small number: {}", n),
//         &Token::Number(n) if n % 2 == 0 => println!("(borrowed) Even number: {}", n),
//         &Token::Ident(ref s) => println!("(borrowed) Identifier: {}", s),
//         &Token::Eof => println!("(borrowed) End"),
//         _ => println!("(borrowed) Other"),
//     }
// }

// // 4) Mutate the inner String via &mut Token using ref mut
// fn mutate_ident(tok: &mut Token) {
//     match tok {
//         &mut Token::Ident(ref mut s) => {
//             s.push_str("_mutated");
//         }
//         _ => {}
//     }
// }

// fn main() {
//     println!("-- describe_owned examples --");
//     describe_owned(Token::Number(5));   // prints Small number: 5 (1..=10)
//     describe_owned(Token::Number(8));   // prints Small number: 8 because the 1..=10 arm is first
//     describe_owned(Token::Number(11));  // prints Other (not small, not even)
//     describe_owned(Token::Ident("alice".to_string())); // Identifier: alice
//     describe_owned(Token::Eof);         // End

//     println!("\n-- describe_reordered example --");
//     describe_reordered(Token::Number(8)); // prints Even number: 8 because even arm is first

//     println!("\n-- borrowing and reuse --");
//     let t = Token::Ident("bob".to_string());
//     describe_borrowed(&t); // we pass a reference, so `t` is not moved and can be reused
//     describe_borrowed(&t); // still available

//     println!("\n-- mutate via &mut --");
//     let mut t2 = Token::Ident("start".to_string());
//     mutate_ident(&mut t2);    // modifies inner String
//     describe_borrowed(&t2);   // shows mutated value
// }







// enum Connection {
//     Disconnected,
//     Connecting{retries: u32},
//     Connected{ id: u64},
//     Closed,
// }

// impl Connection {
//     fn next(self)-> Connection{
//         match self {
//             Connection :: Disconnected => Conection::Connecting{ retries: 0},
//             Connectionn:: Connecting {retries} if retries <3 => Connection::Connecting{retries: retries +1},
//             Connection :: Connecting {...} => Connection::Closed,
//             Connection ::Connected {id}=> {
//                 println!("Already connected as {}", id);
//                 Connection :: Connected {id};

//             }
//             Connection::Closed => Connection::Disconnected,
//         }
//     }
// }


fn main() {
    use Connection::*;

    // Start disconnected
    let mut conn = Disconnected;

    for _ in 0..7 {
        println!("Current state: {:?}", conn);
        conn = conn.next(); // advance to next state
    }

    // Example of staying connected
    let mut connected = Connected { id: 42 };
    connected = connected.next(); // prints message, stays connected
    println!("Still connected: {:?}", connected);
}

#[derive(Debug)]
enum Connection {
    Disconnected,
    Connecting { retries: u32 },
    Connected { id: u64 },
    Closed,
}

impl Connection {
    fn next(self) -> Connection {
        match self {
            Connection::Disconnected => Connection::Connecting { retries: 0 },
            Connection::Connecting { retries } if retries < 3 => {
                Connection::Connecting { retries: retries + 1 }
            }
            Connection::Connecting { .. } => Connection::Closed,
            Connection::Connected { id } => {
                println!("Already connected as {}", id);
                Connection::Connected { id }
            }
            Connection::Closed => Connection::Disconnected,
        }
    }
}
