// //reference counted smart pointer
// // allows multiple ownership by keeping track of the number of references to a value in heap memory
// // automatically deallocates the value when the reference count goes to zero
// // like shared pointer in C++
// // not a mutable smart pointer
// // only works in single-threaded scenarios
// // use Arc<T> for multi-threaded scenarios
// use std::rc::Rc;

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
// let b = Cons(3, Rc::clone(&a));         // cloning increases the reference count
// let c = Cons(4, Rc::clone(&a));




// Interior Mutability : allows to mutate data even when there are immutable references to that data
// Rust uses unsafe internally inside these types to bypass the usual compile-time checks
// use std::cell::RefCell;

// fn main() {
//     let x = RefCell::new(5); // immutable outer value
//     *x.borrow_mut() += 1;    // mutating inner value
//     println!("{}", x.borrow()); // prints 6
// }

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
    
    *value.borrow_mut() += 10; // mutate through RefCell
    println!("{:?}", a); // shows updated value
}
