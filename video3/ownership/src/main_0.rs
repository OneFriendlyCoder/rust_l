// fn main() {
//     // let mut s = "Hello";
//     // s.push_str("_world0");                         //this can't be mutated, thus push_str is not defined for &str 
//     // let mut s1 = String::from("Hello");          //this string can be mutated
//     // s1.push_str("_world");
//     // println!("{}", s1); 


//     //Integers are simple values and are stored on the stack.
//     // When we assign an integer to another variable, it copies the value.
//     // This is called a "copy" type.
//     // If we change the value of one variable, it does not affect the other. 
//     // let mut x = 5;
//     // let y = x;
//     // x += 1; 
//     // println!("x: {}, y: {}", x, y);

//     //this is a move
//     let s1 = String::from("Hello");     
//     let s2 = s1;            //s1 is "moved" to s2, not copied and this operation is inexpensive
//     println!("The s1 string is : {}", s2);


//     //this is a clone
//     let s3 = String::from("Baby");
//     let mut s4 = s3.clone();
//     s4.push_str("girl");
//     println!("The s3 : {}, s4 : {}", s3,s4);

//     // println!("Hello, world!");
// }


//OWNERSHIP AND FUNCTIONS

fn main() {
    let s = String::from("hello");

    // takes_ownership(s);     //s value moves in the function and is not valid here, 's' goes out of scope here and the value of the trio [ptr,len, capacity] in the main function is invalidated
    // println!("This will not work: {}", s); // this will cause an error as s is no longer valid
    // let x = 5;  
    // makes_copy(x);          //x will move into the function but i32 it is a copy so can be used here as well
    // println!("The value of x is: {}", x); // this will work as i32 is a copy type and x is still valid 

    let s1 = gives_ownership(); // gives ownership of the string to s1, s1 comes into scope

    let s2 = String::from("world");     //s2 comes into scope

    let s3 = takes_and_gives_back(s2); // takes ownership of s2 and gives it back, so s3 is valid now

}


fn takes_ownership(some_string: String){
    println!("The string is: {}", some_string);         // some_string contains the stack part of the String i..e ptr, capacity and length
}

fn makes_copy(some_int: i32){
    println!("The integer is: {}", some_int);
}  

// returning a values from function also transfers ownership
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}


fn takes_and_gives_back(a_string: String) -> String {
    a_string
}