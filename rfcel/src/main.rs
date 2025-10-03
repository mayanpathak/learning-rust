// fn main() {
//     println!("Hello, world!");
// }


// fn main (){
//     println!("Hello, world!");
// }

// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(5);
//     println!("value = {}", *a);
//     println!("strong_count = {}", Rc::strong_count(&a));

//     let b = Rc::clone(&a);
//     println!("after clone, strong_count = {}", Rc::strong_count(&a));

//     // both drop at end of scope; when count reaches 0, value is dropped
// }




// use std ::cell::RefCell;

// fn main (){
//     let data = RefCell::new(10);
//     {*data.borrow_mut() += 5;}
// println!("data = {}", *data.borrow());


// }



use std ::{rc::Rc, cell::RefCell};
fn main (){
    let counter = Rc::new(RefCell::new(0));
    let c1 = Rc::clone(&counter);
    let c2 = Rc::clone(&counter);


    *c1.borrow_mut()+=1;
        *c2.borrow_mut() += 10;

     println!("counter = {}", *counter.borrow()); // prints 11
    println!("strong_count = {}", Rc::strong_count(&counter)); // prints 3

}