// defining structs that can take generic types

struct Point<T>{
    x: T,
    y: T,
}

struct Point1<T,U>{     //to define a Point whose both data are generics we use multiple generic type params
    x: T,
    y: U,
}

// generic types in Enum definations
enum Option<T>{
    Some(T),            //this hold value of type T
    None                // does not hold any value
}

enum Result<T,E>{           // this is an enum with 2 generics
    Ok(T),
    Err(E),
}

fn main(){
    let int_point = Point{x: 5, y: 10};
    let float_point = Point{x: 5.5, y: 11.9};
    // let mixed_point = Point{x: 4, y:11.1};             // this wont work as mismatched types
    let mixed_point = Point1{x: 4, y:11.1};
}