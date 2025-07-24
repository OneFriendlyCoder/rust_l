// REFERENCES AND BORROWING 

fn main() {
    let mut s1 = String::from("Hello");

    // change(&s1);
    change_mutable(&mut s1);
    let len = calculate_length(&s1);                // passing a reference to s1, so it is not moved
    println!("The length of '{}' is {}.", s1, len);
}


fn calculate_length(s: &String) -> usize { // s is a reference to a String. 's' contains the pointer to s1 and not the actual String "hello"
    s.capacity() // returning the length of the string
}

// making changes to borrowed values is not allowed
// fn change (some_String: &String) {
//     some_String.push_str(", world!"); // this will not compile because we are trying to change a borrowed value
//     // even though some_String points to a memory location in the stack which is not the actual string, the function .push_str can change the string value, if it is a mutable reference to the string.
// }

fn change_mutable(s: &mut String) {
    s.push_str(", world!");
}
// since 's' is a reference, we can still use 's1' after the function call
// also since it is a reference, we do not take ownership of the String, so it is not moved and nothing happens when it goes out of scope
// reference are immutable, by default
// we can only have one mutable reference to a value in a particular scope
// we can define a scope using the curly braces {} use the mutable references in that scope and then go out of the scope
// we can also not have a mutable reference and an immutable reference to the same value in the same scope
// multiple immutable references are allowed in the same scope
// The last usage of a mutable reference must be after the last usage of an immutable reference to the same value in the same scope


// REFERENCES RULES : 
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.