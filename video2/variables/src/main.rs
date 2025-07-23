// const can be only assigned to constant expressions

fn main() {
    // println!("Hello, world!");
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x += 1;
    // println!("The value of x is: {}", x);

    //shadowing variables (without using mut and using let keyword), we can even change the type of the variable when using shadowing, cz we are creating a new variable with the same name
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    another_function();
}  

fn another_function() {
    // This function is just a placeholder to demonstrate that we can have multiple functions.
    println!("This is another function.");
}