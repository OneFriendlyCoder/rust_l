// iterating over the values in a vector

fn main() {
    let mut v = vec![1,2,3,4,5];
    for i in &v {                       //iterating over refer
        println!("{}", i);
    }

    for i in &mut v {           //changing the values in the vector
        *i+=20;
    }

    // using an enum to store multiple types
    enum Spreadsheetcell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Spreadsheetcell::Int(10),
        Spreadsheetcell::Float(10.5),
        Spreadsheetcell::Text(String::from("Hello")),
    ];


    for i in &row {
        match i {           // i is the refer to the whole enum value, 
            Spreadsheetcell::Int(value) => println!("Integer: {}", value),
            Spreadsheetcell::Float(value) => println!("Float: {}", value),
            Spreadsheetcell::Text(value) => println!("Text: {}", value),
        }
    }

}