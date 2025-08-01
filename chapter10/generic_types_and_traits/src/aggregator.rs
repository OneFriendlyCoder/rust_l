// traits : A type's behaviour consits of the methods we can call on that type. Different types share the same behaviour if we can call the same methods on all of those types.
// traits are a way to group method signatures together to define a set of behaviours necessary to accomplish some purpose

// pub trait Summary {                 // pub for other crates to use this trait
//     fn summarize(&self) -> String;          //the function signature, the body would be implemented by each type that uses this trait
//     // we can have muliple function signature for a single trait

// }


//default behaviour for some or all methods in a trait instead of requiring implementations for all methods on every type
pub trait Summary{          // Summary trait has been defined on a data type
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String{
        format!("Read more from : {}", self.summarize_author())
    }
}


//defining functions that accept different types using traits 
// pub fn notify(item: &impl Summary){
//     println!("Breaking news! {}", item.summarize());
// }

//or
pub fn notify<T: Summary>(item: &T){            //trait Bound syntax
    println!("Breaking news!", item.summarize());
}


//specifying multiple traits using + syntax
pub fn notify(item: &(impl Summary + Display)) {}
//or
pub fn notify<T: Summary + Display>(item: &T){}


//clearer trait bounds with where clause
fn some_function<T:Display + Clone, U:Clone + Debug>(t: &T, u:&U) -> i32{}
//or
fn some_function<T,U>(t: &T, u: &U) -> i32
    where
    T: Display + Clone,
    U: Clone + Debug,
{}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//To use a default implementation to summarize instances of NewsArticle
// impl Summary for NewsArticle {}              // using this again is giving me error as there are 2 implementations of Summary for NewsArticle, we need only one, this is for default implementations

impl Summary for NewsArticle{
    // fn summarize(&self) -> String{
    //     format!("{}, by {} at ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String{
        format!("@{}", self.author)
    }
}

// pub struct SocialPost{
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub repost: bool,
// }
// impl Summary for SocialPost {}
// // impl Summary for SocialPost{
// //     fn summarize(&self) -> String{
// //         format!("{}: {}", self.username, self.content)
// //     }
// // }


//fn summarize(&self) -> String;  => self refers to the instance of any data type that is using the trait, &self is an immutable pointer

//Note : 
// One restriction to note is that we can implement a trait on a type only if either the trait or the type, or both, are local to our crate 


//Default implementations can call other methods in the same trait. Even if those methods do not have a default implementaiton



//functions that returns traits, but this can be used to only return a single type and not multiple types
// fn returns_summarizable() -> impl Summary{           //this returns the type SocialPost that implements the trait Summary
//     SocialPost{...}
// }


