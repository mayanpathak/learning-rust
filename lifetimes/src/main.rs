// fn second_word (s: &str)-> &str {
//     let mut iter = s.split_whitespace();
//     iter.next();
//     iter.next().unwrap_or("")
// }


// fn main (){
//     println!("{}", second_word("hello world rust"));
//     println!("{}", second_word("hello Mayan"));
//     println!("{}", second_word(""));
// }


// fn merge_first<'a>(a: &'a str, _b:&str)->&'a str{
//     a
// }


// fn main (){
//     let s1 = "hello";
//     let s2 = "world";
//     let result = merge_first(s1, s2);
//     println!("{}", result);
// }


// struct Borrowed <'a>{
//     v:&'a Vec<i32>,
// }



// impl <'a> Borrowed <'a> {
//     fn first(&self)-> &i32{
//         &self.v[0]
//     }
// }


// fn main (){
//     let number = vec![10, 20, 30];
//     let Borrowed = Borrowed{ v: &number};
//     println!("first element: {}", Borrowed.first());
// }


// struct Borrowed<'a> {
//     v: &'a Vec<i32>,
// }

// impl<'a> Borrowed<'a> {
//     fn first(&self) -> &i32 {
//         &self.v[0]
//     }
// }

// fn main() {
//     let numbers = vec![10, 20, 30];
//     let borrowed = Borrowed { v: &numbers };
//     println!("First element: {}", borrowed.first()); // 10
// }



// struct Pair <'a>{
//     x:&'a i32,
//     y:&'a i32,
// }

// impl <'a> Pair <'a> {
//     fn sum (&self) -> i32 
//     {
//             *self.x + *self.y
//         }
//     }


//     fn main(){
//         let a = 5;
//         let b = 10;
//         let pair = Pair{ x: &a, y: &b};

//         println!("sum = {}", pair.sum());
//     }



// fn longest <'a> (x: &'a str, y: &'a str) -> &'a str{
//     if x.len() > y.len() {x} else {y}
// }

// fn main (){
//      let s1 = String :: from ("short");
//      let s2 = String :: from ("longer");

//      let result = longest(&s1, &s2);

//      println!("longest string : {}", result);
// }




// struct Multi <'a, 'b>  {
//     first : &'a str,
//     second: &'b str,
// }


// fn main (){
//     let s1 = String :: from("apple");

//      {
//         let s2 = String::from("banana");
//         let both = Multi { first: &s1, second: &s2 };
//         println!("{} and {}", both.first, both.second);
//     }

// }

// struct Text<'a> {
//     content: &'a str,
// }

// impl<'a> Text<'a> {
//     fn first_word(&self)-> &str{
//         self.content.split_whitespace().next().unwrap()
//     }
// }


// fn main (){
//     let s = String::from("hello worlds");
// let text = Text {content: &s};
// println!("first word: {}", text.first_word());

// }

// struct Container<'a>{
//     values:&'a [i32],
// }

// impl<'a> Container <'a>{
//     fn get (&self, index:usize)-> Option<&i32>{
// self.values.get(index)    
//         }
//     }
// fn main (){
//     let nums = vec![10, 20, 30,];
//     let container = Container{ values: &nums };
//     println!("vlue at index 1: {:?}", container.get(2))
// }



// fn choose_first <'a> (x: &'a str, _y: &str)-> &'a str {
//     x
// }

// fn main (){
//     let s1 = String:: from("alpha");
//     let s2 = String:: from("beta");
//     let result = choose_first(&s1,&s2);
//     println!("chosen: {}", result);
// }




// struct Processor<'a> {
//     op: Box<dyn Fn(&'a str) -> usize + 'a>,
// }
// fn main (){
//     let s = String :: from("rustt");
//     let proc = Processor {op: Box::new(|x: &str| x.len())};
//     println!("length: {}", (proc.op)(&s));
// }


// struct Part<'a> {
//     piece: &'a str,
// }

// struct Whole<'a, 'b> {
//     part: Part<'a>,
//     another: &'b str,
// }

// fn main() {
//     let a = String::from("engine");
//     let b = String::from("wheels");

//     let car = Whole { part: Part { piece: &a }, another: &b };
//     println!("Car parts: {} and {}", car.part.piece, car.another);
// }


struct Holder <'a,T>
where T: 'a,
{
    value: &'a T,
}

impl<'a, T> Holder <'a,T>

where T:'a,
{
    fn get(&self) -> &T{
        self.value
    }
}

fn main(){
    let num = 42;
    let holder = Holder{ value: &num};
    println!("Held value: {}",holder.get());
}



