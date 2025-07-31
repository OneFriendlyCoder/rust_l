//Generic types : Option<T,E>: T is the generic type
// we can express the behaviour of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code

// removing duplication by extracting a function, same as a function does
// fn main(){
//     let number_list = vec![34,50,25,100,65];
//     let mut largest = &number_list[0];

//     for number in number_list{
//         if number > largest{
//             largest = number;
//         }
//     }

//     println!("The largest number is : {}", largest);
// }

//generic data types : can be created for functions and data types like structs
// generics in function definations : while defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return values

//though the same code and logic for finding the largest number but the code is repeated, so we will remove the duplicate code using generics
// fn largest_i32(list: &[i32])-> &i32{
//     let mut largest = &list[0];
//     for i in list{
//         if i > largest{
//             largest = i;
//         }
//     }
//     largest
// }

// fn largest_char(list: &[char]){
//     let mut largest = &list[0];
//     for i in list{
//         if i > largest{
//             largest = i;
//         }
//     }
//     largest
// }

// fn main(){
//     let number_list = vec![34,32,56,78,99,1];
//     let nresult = largest_i32(&number_list);
//     println!("The largest number is {nresult}");

//     let char_list = vec!['y','m','n','a','k'];
//     let cresult = largest_char(&char_list);
//     println!("The largest char is {cresult}")
// }



//Generics to remove duplicate code
// fn largest<T>(list: &[T]) -> &T   => the function largest is generic over some type T

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in list {
        if i > largest {            //this wont compile for all the values of T, because we do not know the type of the data and this can be only used for the data that can be ordered
            // this can be enabled using the PartialOrd trait from the std::cmp::PartialOrd
            largest = i;
        }
    }
    i
}

fn main(){
    let nlist = vec![1,2,11,32,4,0];
    let result = largest(&nlist);
    println!("The largest number is : {}", result);
    
    let clist = vec!['a','b','d','h'];
    let result = largest(&clist);
    println!("The largest char is : {}", result);
}