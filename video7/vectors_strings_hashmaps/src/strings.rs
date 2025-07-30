//Strings : owned and borrowed
// They are UTF-8 encoded so we can include any properly encoded data in them 


fn main() {
    let mut s1 = String::new();

    let data = "Mahesh Dalle";
    let s2 = data.to_string();       //available on any type that implements the Display trait

    let s3 = String::from("Hello ");

    // Updating a string
    // using push_str
    s1.push_str("Hello, ");     // push_str takes the string slice
    s1.push_str("world!");
    println!("s1: {}", s1);

    //update via the + Operator or format! macro
    let s4 = s3 + &s2; // s3 is moved, so we cannot use it after this. This may look like we are copying a lot of values but it is simply passing the ownership
    println!("s4: {}", s4);

    //concatenating multiple strings
    let s5 = String::from("tic");
    let s6 = String::from("tac");
    let s7 = String::from("toe");
    let s8 = s5 + "-" + &s6 + "-" + &s7; // we can use the + operator to concatenate multiple strings
    let s9 = format!("{}-{}-{}", s5, s6, s7); // format! macro is more flexible and does not take ownership of the strings
    println!("s8: {}", s8);

    //Indexing into Strings
    // we can not access parts of Strings using indexing in RUST, but why? 
    //Internal representation of String
    // Strings are UTF-8 encoded, so a single character can take up to 4 bytes
    // If we try to access a character using indexing, we may end up slicing the string
    let s10 = String::from("Hello");
    // let h = s10[0];

    // There are 3 ways to look into string in RUST, since they are in UTF-8 encoding
    // bytes, scalars and grapheme clusters


    // slicing strings : we can use [] with a single number and a range to create a string slice containing particular bytes


    //methods for iterating over strings : accesssing elements of a string
    //best way to do so is using the chars() method
    for c in "Зд".chars() {
        println!("{c}");
    }

    // we can also use the bytes() method to iterate over the bytes of a string
    for b in "Зд".bytes() {
        println!("{b}");
    }

}


