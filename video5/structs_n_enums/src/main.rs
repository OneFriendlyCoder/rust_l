
// WITHOUT USING STRUCTS
// fn main() {
//     let w = 20;
//     let h = 50;
//     println!("The area of the rectangle is: {}", area(w, h));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }


// WITH STRUCTS
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect = Rectangle {
//         width: 20,
//         height: 50,
//     };

//     println!("The area of the rectangle is: {}", area(&rect));
//     // println!("The rectangle is: {}", rect);         // this will not work without implementing the Display trait, bcz it is ambiguous how to display a struct
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }



// ADDING USEFUL FUNCTIONALITIES WITH DERIVED TRAITS
// the debug trait allows us to print the struct directly
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {        // impl block is used to define methods for structs, traits and enums
    fn area(&self) -> u32 {             // self is a reference to the instance of the struct, allowing us to access its fields
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
// methods can borrow self immutably or mutably, but not both at the same time, just like with function parameters

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 100,
        height: 45,
    };

    println!("The rectangle is: {:?}", rect); // using debug trait to print the struct, {:?} is a format specifier for debug traits and is used to print the values in a user friendly manner
    println!("The area of the rectangle is: {}", rect.area()); // using the area method of the Rectangle struct

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}