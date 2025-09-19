// pub fn append_hello_world(string: &mut String) -> &str {
//     string.push_str("   Hello World");
//     string .as_str()
// }


// fn main () {
//     let mut new_string = String:: from("heyy mayan");
//     let result = append_hello_world(&mut new_string);
//     println!("Result: {}", result);
// }


fn main() {
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];

    {
        let first = &vector[0];
        println!("First element: {}", first);
    }

    vector.push(7); // ✅ now pushing an i32
    println!("{:?}", vector);
}
