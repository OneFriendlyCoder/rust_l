// match with Options<T>


fn main(){
    let five = Some(5);
    let six = plus_One(five);
    let none = plus_One(None);
    println!("Six: {:?}, None: {:?}", six, none);           //println! can print Option<T> values, as it directly uses the format specifier {:?}

    let points = allot_points("three");
    let points = allot_points("four"); // this will match the _ placeholder and return 0
    println!("Points: {}", points);
}
 
fn plus_One(x: Option<i32>) -> Option<i32> {
    match x {                   //matches are exhaustive and we have to handle all cases
        Some(i) => Some(i+1),
        None => None,
    }
}



// The _ Placeholder
// The _ placeholder can be used to ignore values in a match arm, it is useful when you want to match a value but don't care about it

fn allot_points(x: &str) -> u8 {
    match x {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        _ => 0, // the _ placeholder matches any value not matched by the previous arms
    }
}
