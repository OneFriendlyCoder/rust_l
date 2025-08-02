// lifetimes : another kind of generics, this ensures that references are valid as long as we need them to be

//Main aim of lifetimes : preventing dangling references with lifetimes
// fn main() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("r:{r}");
// }

// fn main() {
//     let result;
//     {
//         let string1 = String::from("abcd");
//         let string2 = "xyz";
//         let result = longest(string1.as_str(),string2);
//     }   //string 1 and string2 are dropped here, results points to free memory

//     println!("The longest string is {result}");
// }

// // the below function is errorenous as and we can't let the caller decide to deal with any reference that the function passes, as the reference might go out of scope
// fn longest(x: &str, y: &str) -> &str{
//     if x.len() > y.len() {x} else {y}
// }
// rust does not have a GC
// rust does not do runtime checks for dangling pointers
// rust enforces lifetime rules at compile time



// Lifetime Annotation Syntax
// lifetime annotation does not change how long any references live, they rather describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
// Like functions can accept any type when the signature specifies a generic type parameter, similarily functions can accept references with any lifetime by specifying a generic lifetime parameter.

// &i32             : a reference
// &'a i32          : a reference with an explicity lifetime
// &'a mut i32      : a mutable reference with an explicity lifetime


// Lifetime annonations in Function Signatures 
//keynote : the rust compilers wants a guarentee that the returned reference wont outlive the input references

// this function takes referecnecs to string1 and string2
// both references are annotated with the same lifetime 'a
// it returns a reference to one of them
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{      //'a is a label for all lifetimes of x,y and return values
    if x.len() > y.len() {x} else {y}
}


fn main() {
    let result;
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";
        result = longest(string1.as_str(),string2);
    }// string1 and string2 goes out of scope
    //result is used after that block
    //bcz we have used the 'a annotation compiler know that result can't live longer than string1 or string2 and flags the error
    println!("The longest string is {result}");
}


//when do we need to specify lifetimes?
//when returning references or storing them in structs


//the lifetime elision rules only applied to the reference parameters
// 1. each parameter gets its own lifetime
// 2. if there's one input lifetime, its assigned to the output
// 3. if one of the parameters is &self or &mut self, its assigned to the output

//ex : 
// 1 
fn foo(x: &str, y: &str) -> &str        // this will give error as each parameter gets its own lifetime and this confuses RUST to return what lifetime be it for the 1st param of the 2nd param
fn foo<'a>(x: &'a str, y: &'a str) -> &'a str       // all lifetimes are annotated with 'a thus the problem is resolved

//2 : the rules only apply to the reference params
fn first_word(s: &str, index:i32) -> &str{     //one input &str -> 'a and the return value has the same 'a lifetime
    s.split_whitespace().next().unwrap();
}

//3 then the return output reference lifetime is same as self
impl Book{  
    fn title(&self) -> &str{
        &self.name
    }
}


// some combinations:
// case1:
// rule 3 applies, return lifetime is assumed to be same as self
impl Book {
    fn title(&self, author: &str) -> &str {
        &self.name
    }
}


//case 2: 
// rule 3 kicks in and rust will assume we are returning something from self
// but author lifetime is a mismatch from self lifetime
// we have to explicitly tell the function to return the lifetime of author by lifetime annotation
//correct code : 
// fn title<'a>(&self, author: &'a str) -> &'a str {
//     author
// }
// incorrect code : 
impl Book {
    fn t
    itle(&self, author: &str) -> &str {
        author
    }
}


//case 3:
// rule 1 kicks in, we have to specify the lifetime for each reference
impl Book {
    fn title(&self, author: &str) -> &str {
        if self.name.len() > author.len() {
            &self.name
        } else {
            author
        }
    }
}


