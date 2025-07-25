fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);  // pass a reference
    // let (word2, word3) = second_word(&s); // pass a reference
    println!("The first word is : {}", word);
}

// how can we return a substring from a string?
// fn first_word(s: &String) -> usize {        // usize is a copy variable, so even though the scope of s_len ends after the function its value is being copied to the return value
//     let bytes = s.as_bytes();       // as_bytes() converts the string to bytes array
//     let s_len = s.len();          // get the length of the string
//     for (i, &item) in bytes.iter().enumerate() {            //enumerate() wraps the iterator in a tuple (index, value)
//         if item == b' ' {
//             return i;               // could explicitly return here, using the return keyword
//         }
//     }
//     // s.len()         // return without the return keyword
//     s_len           // thus this will not throw an error, returning only a reference will cause error as it will be a dangling pointer
// }


fn second_word(s: &String) -> (usize, usize){
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start == 0 {
                start = i + 1; // skip the space
            } else {
                end = i; // set end to the current index
                break; // exit loop after finding the second word
            }
        }
    }

    if end == 0 { // if no second word found, set end to the length of the string
        end = s.len();
    }

    (start, end) // return a tuple with start and end indices
}

//string slice : A string slice is a reference to part of a String
// similar to how we pass the reference to the String, we can do the same with slices

fn first_word(s: &String) -> &str{          //&str is the type of a string slice
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]              //return the whole string if no space is found
}