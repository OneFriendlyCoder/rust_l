// passing two values using a tuple

fn main() {
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1); // s1 is moved into the function, and s2 is returned with the length
    println!("The length of '{}' is {}.", s2, len);
}


fn calculate_length(s:String) -> (String, usize) {
    let length = s.len();
    (s, length) // returning a tuple containing the string and its length
}