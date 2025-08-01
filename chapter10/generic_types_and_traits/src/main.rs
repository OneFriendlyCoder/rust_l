mod aggregator;                 //declare that there is a module named aggregator and its code is in aggregator.rs or aggregator/mod.rs
use aggregator::{SocialPost, Summary};

fn main(){
    let post = SocialPost{
        username: String::from("Abhyanand"),
        content: String::from("This content is so crazy"),
        reply: false,
        repost: false,
    };
    println!("1 new social post: {}", post.summarize());
}