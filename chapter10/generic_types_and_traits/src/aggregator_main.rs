mod aggregator;                 //declare that there is a module named aggregator and its code is in aggregator.rs or aggregator/mod.rs
// use aggregator::{SocialPost, Summary};
// use aggregator::{SocialPost,NewsArticle, Summary};
use aggregator::{NewsArticle, Summary};
fn main(){
    // let post = SocialPost{
    //     username: String::from("Abhyanand"),
    //     content: String::from("This content is so crazy"),
    //     reply: false,exi
    //     repost: false,
    // };
    // println!("1 new social post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
}