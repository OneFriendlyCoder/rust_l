//using tuple structs without named fields to  create different types
// tuple structs : structs that look similar to tuples (just the type of fields and not there names)

struct Point(i32, i32, i32);        // even though Point and Color has same types, they are different types, so each tuple struct we define is a type on its own
struct Color(u8, u8, u8);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let (x,y,z) = origin;           //destructuring the tuple struct Point
    // println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
    // println!("Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);

    //unit like structs
    //unit like structs are structs that have no fields, they are used to implement traits on

    //ownership of a struct
    

}   