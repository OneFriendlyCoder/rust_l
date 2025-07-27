
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

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 50,
    };

   println!("The rectangle is: {:?}", rect); // using debug trait to print the struct, {:?} is a format specifier for debug traits and is used to print the values in a user friendly manner
}