// // Box<T> is a smart pointer
// // defining a self referential data structure
// enum List {
//     Cons(i32, Box<List>),   //Cons(...) is a variant and not a type, so we need to use Box with the data type and not a variant
//     Nil,
// }

// use crate::List::{Cons, Nil};          // not an external crate, it means the current crate

// fn main(){
//     // let list = Cons(1, Cons(2, Cons(3, Nil)));
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

// // Recursive data types = Self referential types



// Treating Smart Pointers like Regular References with the Deref Trait
// Implmenting the Deref Trait allows us to customize the behavior of the dereference operator (*) for our smart pointer type

// // normal pointer
// fn main(){
//     let x = 5;
//     let y = &x;
//     assert_eq!(5, x);
//     assert_eq!(5, *y);
//     // assert_eq!(5, y); 
// }


// // Box pointer
// fn main(){
//     let x = 5;
//     let y = Box::new(x);
//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }


// struct MyBox<T>(T);
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }


// implementing Deref trait for MyBox
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);
//     assert_eq!(5, x);
//     assert_eq!(5, *y);          // *(y.deref()) -> code ran by the rust compiler
// }


// Deref coercion
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);         // &MyBox<String> -> &String -> &str
}
// without deref coercion
// fn main() {
//     let m = MyBox::new(String::from("Rust"));
//     hello(&(*m)[..]);
// }