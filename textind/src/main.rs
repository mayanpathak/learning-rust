// pub struct TextIndexer<'a> {
//     text: &'a str,
//     words: Vec <&'a str>,

// }

// impl<'a> TextIndexer<'a>{
// pub fn new(text:String)-> TextIndexer<'a>{
//     TextIndexer{
//         text,
//         words:Vec::New(),
//     }
// }

// pub fn index_words(&'a mut self){
//     let text_ref: &'a str = &self.text;

//     self.words = text_ref.split_whitespace().collect();

// }
// pub fn words(&self)-> &Vec<&'a str>{
//     &self.words
// }
// }


// fn main (){
//     let mut indexer = TextIndexer:: new(
//                 "Rust makes lifetimes explicit and safe".to_string(),
//     );
//     indexer.index_words();
//     for word in indexer.words(){
//         println!("{}", word);
//     }}


// pub struct TextIndexer<'a> {
//     text: String,
//     words: Vec<&'a str>,
// }

// impl<'a> TextIndexer<'a> {
//     pub fn new(text: String) -> TextIndexer<'a> {
//         TextIndexer {
//             text,
//             words: Vec::new(),
//         }
//     }

//     pub fn index_words(&'a mut self) {
//         // Split text into words and store references
//         let text_ref: &'a str = &self.text;

//         self.words = text_ref
//             .split_whitespace()
//             .collect();
//     }

//     pub fn words(&self) -> &Vec<&'a str> {
//         &self.words
//     }
// }
// fn main() {
//     let mut indexer = TextIndexer::new(
//         "Rust makes lifetimes explicit and safe".to_string(),
//     );

//     indexer.index_words(<&'a str>);

//     for word in indexer.words() {
//         println!("{}", word);
//     }
// }


// pub struct TextIndexer<'a> {
//     text: String,
//     words: Vec<&'a str>,
// }

// impl<'a> TextIndexer<'a> {
//     pub fn new(text: String) -> TextIndexer<'a> {
//         TextIndexer {
//             text,
//             words: Vec::new(),
//         }
//     }

//     pub fn index_words(&'a mut self) {
//         // Borrow the text as a &str
//         let text_ref: &'a str = &self.text;

//         // Collect slices into words
//         self.words = text_ref.split_whitespace().collect();
//     }

//     pub fn words(&self) -> &Vec<&'a str> {


// pub struct TextIndexer<'a> {
//     text: String,
//     words: Vec<&'a str>,
// }

// impl<'a> TextIndexer<'a> {
//     pub fn new(text: String) -> TextIndexer<'a> {
//         TextIndexer {
//             text,
//             words: Vec::new(),
//         }
//     }

//     pub fn index_words(&'a mut self) {
//         // Borrow the text as a &str
//         let text_ref: &'a str = &self.text;

//         // Collect slices into words
//         self.words = text_ref.split_whitespace().collect();
//     }

//     pub fn words(&self) -> &Vec<&'a str> {
//         &self.words
//     }
// }

// fn main() {
//     let mut indexer = TextIndexer::new(
//         "Rust makes lifetimes explicit and safe".to_string(),
//     );

//     indexer.index_words(); // ✅ no arguments needed

//     for word in indexer.words() {
//         println!("{}", word);
//     }
// }


// pub struct TextIndexer<'a> {
//     text: String,
//     words: Vec<&'a str>,
// }

// impl<'a> TextIndexer<'a> {
//     pub fn new(text: String) -> TextIndexer<'a> {
//         TextIndexer {
//             text,
//             words: Vec::new(),
//         }
//     }

//     // ✅ remove `'a` from &mut self
//     pub fn index_words(&mut self) {
//         let text_ref: &str = &self.text;

//         self.words = text_ref.split_whitespace().collect();
//     }

//     pub fn words(&self) -> &Vec<&'a str> {
//         &self.words
//     }
// }

// fn main() {
//     let mut indexer = TextIndexer::new(
//         "Rust makes lifetimes explicit and safe".to_string(),
//     );

//     indexer.index_words(); // ✅ just call, no arguments

//     for word in indexer.words() {
//         println!("{}", word);
//     }
// }



pub struct TextIndexer<'a> {
    text: &'a str,
    words: Vec<&'a str>,
}

impl<'a> TextIndexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, words: Vec::new() }
    }

    pub fn index_words(&mut self) {
        self.words = self.text.split_whitespace().collect();
    }

    pub fn words(&self) -> &Vec<&'a str> {
        &self.words
    }
}

fn main() {
    let text = "Rust makes lifetimes explicit and safe".to_string();
    let mut indexer = TextIndexer::new(&text); // borrow text

    indexer.index_words();

    for word in indexer.words() {
        println!("{}", word);
    }
}
