// match allows us to compare a value against a series of patterns and then execute code based on which pattern matches
// to check if all cases are handled
// values go in through each pattern in a match, and at the first pattern the value fits, the associated block of code is executed
// if returns a boolean value, whereas a match can return any type

#[derive(Debug)] // to print the enum value
enum States{
    Alaska,
    California,
    Florida,
    NewYork,
}

enum Coins {
    Penny,
    Nickel,
    Dime, 
    Quater(States),
}
// associated values can be used to store additional information with an enum variant, like the state in the Quater variant
fn value_in_cents(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => {                   
            // println!("This is a penny");
            1
        },                          // pattern => code(expression, in {...} it should return a value) and each arm is separated by a comma, the code associated with each pattern is an expression
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quater(state) => {
            println!("This is a quater from {:?}", state);
            25
        },
    }
}

fn main() {
    let penny = Coins::Penny;
    let quater = Coins::Quater(States::California);
    println!("{}",value_in_cents(quater));
}
