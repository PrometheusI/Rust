// Trait impl without a standart impl
//pub trait Summery{
//    fn summerize(&self) -> String;
//}

pub trait Summary {
    fn summerize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summerize_author())
    }
}

pub struct Newsarticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Newsarticle{
    fn summerize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summerize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Article{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    fn summerize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize())
}
// Allow two types to have different Type
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// Two items must have the same traits
// pub fn notify<T: Summary>(item1: &T, item2: &T) {


// To enforce to have two traits we can use plus
// pub fn notity(item: &impl Summary + Display){

// To make it easier to read we can use the where syntax
// pub fn somefunktion<T, U>(t: &T, u: &U) -> i32
// where T: Display+ Clone,
//       U: Clone + Debug 
// {

// We can also return something with a trait
 fn _return_summarizble() -> impl Summary{
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
 }