// ENUMS : the data type is same, but the values that they take can be only one, not all
// :: is used to access something that is defined iside a namespace or module
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn routes(IpAddrKind) {

// }

// fn main(){
//     let four = IpaddrKind::V4;      //both V4 and V6 are of the same data type
//     let six = IpaddrKind::V6;

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };

// }


// putting data directly into each variant of the enum
// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// enum Message {
//     QuitMessage,
//     MoveMessage {x: i32, y: i32},
//     WriteMessage(String),
//     ChangeColorMessage(i32, i32, i32),
// }

// impl Message {      //we will use Matching to call the enum variants
//     fn call(&self){
//         match self {
//             Message::WriteMessage(text) => {
//                 println!("Writing message: {}", text);
//             }
//             _ => {
//                 println!("Message called");
//             }
//         }
//     }
// }

// fn main() {
//     let home = IpAddrKind::V4(String::from("127.0.0.1"));
//     let loopback = IpAddrKind::V6(String::from("::1"));

//     let m = Message::WriteMessage(String::from("hello"));
//     m.call();
// }



//OPTIONS ENUM
// Null is a value that is currently invalid or absent for some reason
// Option enum : encodes the scenario of a value could be something or nothing
// We do not need to bring Option in the scope explicitly, it is already in the prelude

// enum Option<T> {             // : generic type T, we can provide any type for T
//     Some(T),
//     None,                // right now I have no value but if I will have it would be of type T
// }

fn main(){
    let some_number = Some(5);      
    let some_string = Some("a string"); 
    let absent_number: Option<i32> = None;          // thus here we have to specify the type of None even though it is invalid or not present

    let x: i8 = 5;
    let y: Option<i8> = Some(10);               //this is an Option type and it can not be added to x directly, it can either be Some or None
    println!("Sum of x:{} and y:{}, is {}", x, y, (x+y));
}