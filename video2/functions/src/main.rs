fn main() {
    println!("Hello, world!");
    another_function(5,7);
    expressions();
    println!("The value of five is: {}", five()); // calling a function that returns a value
}


fn another_function(x: u32,y: u32) {                   // we have to put the data type of the parameter
    println!("This is value of x + y is: {}", x+y);
}


fn expressions() {
    // let x = 5;
    let y = {
        let x = 3;
        x + 1               // no semicolon as it is an expression that returns a value, if we put a semicolon it will be a statement and will not return a value
    };
    println!("The value of y is: {}", y); // y will be 4, as it is the result of the expression
}


//function with return values 
fn five () -> u32 {
    5
}
// rust does not support default parameters
// Statments are expressions that does not return a value (let keyword are statements, function declarations are statements, etc.)
// Expressions are statements that return a value => wrapped in curly braces are expressions, calling a macro is an expression
// in an implicit return, when the function does not return any value, it returns the unit type () which is similar to void in other languages