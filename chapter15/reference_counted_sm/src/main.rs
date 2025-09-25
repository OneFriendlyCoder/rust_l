//reference counted smart pointer
// allows multiple ownership by keeping track of the number of references to a value in heap memory
// automatically deallocates the value when the reference count goes to zero
// like shared pointer in C++
// not a mutable smart pointer
// only works in single-threaded scenarios
// use Arc<T> for multi-threaded scenarios
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
let b = Cons(3, Rc::clone(&a));         // cloning increases the reference count
let c = Cons(4, Rc::clone(&a));
