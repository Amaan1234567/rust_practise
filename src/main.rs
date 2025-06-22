use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},by {} ({})", self.headline,self.author,self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}",self.username,self.content)
    }
}

//pub fn notify<T: Summary>(item: &T) {
//    println!("Breaking news! {}",item.summarize());
//}
// below is syntactic sugar for the function written above

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}",item.summarize());
}

// pub fn notify(item: &(impl Summary + Display))
// pub fn notify<T: Summary + Display>(item: &T)

fn some_funtion<T, U>(t: &T,u: &U) -> i32
where 
    T:Display + Clone,
    U: Clone + Debug,
{
    1
}

fn returns_summarizable() -> impl Summary{
    SocialPost {
        username:String::from("horse_ebooks"),
        content:String::from(
            "of course, as you probablly already know, people"
        ),
        reply: false,
        repost: false,
    }
}


struct Pair<T> {
    x: T,
    y: T
}

impl<T:Display + PartialOrd>Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}",self.x);
        }
        else {
            println!("The largest member is y = {}",self.y);
        }
    }
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        repost: false,
    };
    
    println!("1 new social post: {}",post.summarize())
}