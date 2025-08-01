// traits : A type's behaviour consits of the methods we can call on that type. Different types share the same behaviour if we can call the same methods on all of those types.
// traits are a way to group method signatures together to define a set of behaviours necessary to accomplish some purpose

pub trait Summary {                 // pub for other crates to use this trait
    fn summarize(&self) -> String;          //the function signature, the body would be implemented by each type that uses this trait
    // we can have muliple function signature for a single trait

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, by {} at ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost{
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
}


//fn summarize(&self) -> String;  => self refers to the instance of any data type that is using the trait, &self is an immutable pointer