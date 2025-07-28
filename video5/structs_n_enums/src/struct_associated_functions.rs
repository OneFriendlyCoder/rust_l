// Associated functions without the self parameter in structs
// These functions are called "associated functions" and can be called without an instance of the struct
// String::from is an example of an associated function
// Associated functions are often used for constructors that will return a new instance of struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle{
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let this_is_square = Rectangle::square(19);
    println!("The area of the square is : {}",this_is_square.area());
    println!("The square is: {:?}", this_is_square);
}