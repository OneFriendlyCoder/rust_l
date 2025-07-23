// crates.io
// cargo docs --open, creates documentation for the current crate and opens it in the browser


use std::io;        // use is used to bring the io module into scope(prelude). 
use rand::Rng; // rand crate is used to generate random numbers, Rng trait is used to generate random numbers
// Rng trait is a collection of methods that can be used to generate random numbers
use std::cmp::Ordering; // cmp module is used to compare values, Ordering is an enum that represents the result of a comparison

fn main() {
    println!("Guess the number!");
    println!("Please enter your number!");

    let secret_number = rand::rng().random_range(1..101); // generates a random number between 1 and 100
    // println!("The secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();          //new instance of String to hold the input, :: is used to call an associated function
        io::stdin()
            .read_line(&mut guess)                 //pass the pointer to the mutable string
            .expect("Failed to read line");         // expect method on stdin type

        println!("You guessed: {}", guess);
        let guess:u32 = match guess.trim().parse() {        //match is used to handle the result of the parse method, which converts the string to a number
            Ok(num) => num,                          // if the parse is successful, num is the parsed number
            Err(_) => {
                println!("Please enter a valid number!");
                continue;                             // continue to the next iteration of the loop
            },
        };

        match guess.cmp(&secret_number){                //cmp method compares the guess with the secret number and returns an Ordering enum
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }


}

// an associated function is a function that is associated with a type, and can be called using the type's name followed by two colons (::).
// in this case, String::new() creates a new instance of the String type.
// a "method" is a function that is associated with an instance of a type, and can be called using the instance's name followed by a dot (.) and the method name.
// in this case, read_line is a method that is associated with the stdin() instance of

//Ordering is an enum that represents the result of a comparison, it has three variants: Less, Greater, and Equal.
// We can shadow a previous variable by declaring a new variable with the same name, this is useful when we want to change the type of a variable.