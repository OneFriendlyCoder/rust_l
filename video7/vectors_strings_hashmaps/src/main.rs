// vectors : same data type, contiguous memory


fn main() {

    // let mut v: Vec<i32> = Vec::new();           //new empty vector that holds i32 
    
    // //letting RUST infer the type
    // // let mut v1 = Vec::new();
    // let v2 = vec![1, 2, 3, 4, 5]; //vec! macro creates a vector with the given values


    // // updating a vector
    // v.push(10);
    // v.push(20);
    // v.push(30);
    // v.push(40);

    //dropping a vector drops all its elements

    //reading elements of a vector

    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2){         //returns an Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

}

//22:14