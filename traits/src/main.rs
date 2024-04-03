//A trait is a functionality a type has and can share with other types, to define shared behaviour in an abstract way
//Like interfaces in other languages
use std::fmt::Display;
use core::fmt::Debug;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

    println!("1 new tweet: {}", tweet.summarize_v2());

    let default = ToCheckDefaultTrait {
        default: true
    };

    println!("What you print: {}", default.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    //The orphan rule is a part of Rust's coherence rules. Coherence is about making sure that each trait implementation is unique and unambiguous.
    //Orphan rule states that:
        // You can only implement a trait on a type if either the trait or the type is local to your crate.
        // You cannot implement external traits (from external crates) on external types (types from external crates).
}

pub trait Summary { //Defining trait
    // fn summarize(&self) -> String; //add method signature on which trait is to implement
    //We can also have default implementation of traits methods so to not implement in each type seperately
    //If we want to change that, we can overrride their implementations
    fn summarize(&self) -> String {
        String::from("Read More....")
    }

    fn summarize_author(&self) -> String {
        String::from("")
    }

    fn summarize_v2(&self) -> String { //Adding default impl cause we have some types where we no providing impl for these methods eg, 
        String::from("") //impl Summary for ToCheckDefaultTrait {} missing `summarize_v2` in implementation
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub content: String,
    pub author: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize_v2(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct ToCheckDefaultTrait {
    pub default: bool
}

impl Summary for ToCheckDefaultTrait {}


//////////////////////////
//Trait as parameters
//////////////////////////

//We can pass any type that implements Summary trait to this method param, and call any method defined in that given trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//Trait bound syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

//In case we have more than one param to implement that trait
pub fn notify(item1: &impl Summary, item2: &impl Summary);
pub fn notify<T: Summary>(item1: &T, item2: &T); //far better, verbose

//Multiple trait bounds
pub fn notify(item: &(impl Summary + Display));
pub fn notify<T: Summary + Display>(item1: &T); //far better, verbose

//Clearer trait bounds with where clause
fn notify<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32;
fn notify<T, U>(t: &T, u: &U) -> i32 //far clearer
where
    T: Display + Clone,
    U: Clone + Debug
    {}

//////////////////////////
//Trait as return types
//////////////////////////
//We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait
//But that works for single type, not for multiple types
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn returns_summarizable(switch: bool) -> impl Summary { // Compile time error - `if` and `else` have incompatible types
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

//Blanket implementation?????