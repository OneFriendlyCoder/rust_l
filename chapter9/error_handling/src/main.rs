// use std::fs::File;
// use std::io::ErrorKind;
// use std::io::{self, Read};


// //handling recoverable errors using enum Result
// // enum Result<T,E>{            //T,E are generic type params
// //     Ok(T),                   //T is the type of value returned when success with the Ok variant
// //     Err(E),                  //E is the type of value returned when error with the Err variant
// // }

// // fn main(){
// //     let greeting_file_Result = File::open("hello.txt");     // if success greeting_file_Result will have an instance of Ok that contains the file handle
// //     // if fails the greeting_file_Result will hvae an instance of Err which will contain more info about how the error occured

// //     let greeting_file = match greeting_file_Result {
// //         Ok(file) => file,
// //         Err(error) => panic!("Problem opening the file: {error:?}"),
// //     };
// // }


// // matching on different errors
// // fn main(){
// //     let greeting_file_Result = File::open("hello.txt");     // if success greeting_file_Result will have an instance of Ok that contains the file handle
// //     // if fails the greeting_file_Result will hvae an instance of Err which will contain more info about how the error occured

// //     let greeting_file = match greeting_file_Result {
// //         Ok(file) => file,
// //         Err(error) => match error.kind(){
// //             ErrorKind::NotFound => match File::create("hello.txt"){
// //                 Ok(fc) => fc,
// //                 Err(e) => panic!("Problem creating the file: {e:?}"),
// //             },
// //             _ => {
// //                 panic!("Problem opening the file : {error:?}");
// //             },
// //         },
// //     };
// // }


// // we can use unwrap and expect as well


// //propagating errors: returning error to the calling code so that it decide what to do, instead of handling the error in the function itself
// fn main(){

// }
// // fn read_username_from_file() -> Result<String, io::Error>{          //return type is Result<T,E> with T: String and E: io:Error
// //     let username_file_result = File::open("Hello.txt");
// //     let mut username_file = match username_file_result{
// //         Ok(file) => file,       //if successful the "file" is stored in username_file
// //         Err(e) => return Err(e),    //error occured exit the function early with the error
// //     };

// //     let mut username = String::new();
// //     match username_file.read_to_string(&mut username){      //read_to_string(&mut username) -> returns Result<usize, std::io::Error>
// //         Ok(_) => Ok(username),          // if the function succeeds the code calling this will receive an Ok value that holds a String - the username
// //         Err(e) => Err(e),               
// //     }
// // }

// // Ok(file) => file : If username_file_result is Ok(some_file), bind some_file to file and return file from this arm
// // Ok(_) => Ok(username) : Wrapping it in Ok(...) gives you a Result<String, io::Error>—exactly the function’s return type.
// // wrapping in Ok(...)” simply means you’re packaging your successful return value inside the Ok variant of the Result enum so that it matches the function’s declared return type Result<String, io::Error>


// // shortcut for propagating errors: the ? Operator
// fn read_username_from_file() -> Result<String, io::Error>{
//     let mut username_file = File::open("hello.txt");
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;       
//     Ok(username)            //wrapping the success in Ok variant of Result
// }

// // '?' operator converts the error in out desired error format unlike the match expression



// //Chaining ? 
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};


//handling recoverable errors using enum Result
// enum Result<T,E>{            //T,E are generic type params
//     Ok(T),                   //T is the type of value returned when success with the Ok variant
//     Err(E),                  //E is the type of value returned when error with the Err variant
// }

// fn main(){
//     let greeting_file_Result = File::open("hello.txt");     // if success greeting_file_Result will have an instance of Ok that contains the file handle
//     // if fails the greeting_file_Result will hvae an instance of Err which will contain more info about how the error occured

//     let greeting_file = match greeting_file_Result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {error:?}"),
//     };
// }


// matching on different errors
// fn main(){
//     let greeting_file_Result = File::open("hello.txt");     // if success greeting_file_Result will have an instance of Ok that contains the file handle
//     // if fails the greeting_file_Result will hvae an instance of Err which will contain more info about how the error occured

//     let greeting_file = match greeting_file_Result {
//         Ok(file) => file,
//         Err(error) => match error.kind(){
//             ErrorKind::NotFound => match File::create("hello.txt"){
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             _ => {
//                 panic!("Problem opening the file : {error:?}");
//             },
//         },
//     };
// }


// we can use unwrap and expect as well


//propagating errors: returning error to the calling code so that it decide what to do, instead of handling the error in the function itself
fn main(){

}
// fn read_username_from_file() -> Result<String, io::Error>{          //return type is Result<T,E> with T: String and E: io:Error
//     let username_file_result = File::open("Hello.txt");
//     let mut username_file = match username_file_result{
//         Ok(file) => file,       //if successful the "file" is stored in username_file
//         Err(e) => return Err(e),    //error occured exit the function early with the error
//     };

//     let mut username = String::new();
//     match username_file.read_to_string(&mut username){      //read_to_string(&mut username) -> returns Result<usize, std::io::Error>
//         Ok(_) => Ok(username),          // if the function succeeds the code calling this will receive an Ok value that holds a String - the username
//         Err(e) => Err(e),               
//     }
// }

// Ok(file) => file : If username_file_result is Ok(some_file), bind some_file to file and return file from this arm
// Ok(_) => Ok(username) : Wrapping it in Ok(...) gives you a Result<String, io::Error>—exactly the function’s return type.
// wrapping in Ok(...)” simply means you’re packaging your successful return value inside the Ok variant of the Result enum so that it matches the function’s declared return type Result<String, io::Error>


// shortcut for propagating errors: the ? Operator
// fn read_username_from_file() -> Result<String, io::Error>{
//     let mut username_file = File::open("hello.txt");
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;       
//     Ok(username)            //wrapping the success in Ok variant of Result
// }

// '?' operator converts the error in out desired error format unlike the match expression


//Chaining ? operator 
fn read_username_from_file() -> Result<String, io::Error>{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}


//where the ? operator can be used
// ? operator can be used only in functions whose return type is compatible with the value the ? is used on
