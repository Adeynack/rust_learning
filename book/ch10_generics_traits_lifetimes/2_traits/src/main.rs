use std::fmt::{Debug, Display};

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: "Penguins win the Stanley Cup Championship!".to_string(),
        location: "Pittsburgh, PA, USA".to_string(),
        author: "Iceburgh".to_string(),
        content: "The Pittsburgh Penguins once again are the best hockey team in the NHL."
            .to_string(),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());

    notify(&tweet);
    notify(&article);

    notify_desugared(&tweet);
    notify_desugared(&article);

    println!("{:?}", return_summarizable());

    using_trait_bounds_to_conditionally_implement_methods();
}

// Trait `Summary` derives from trait `Debug`
// That makes it mandatory to implement `Debug` or to `#[derive(Debug)]` for
// each implementations of `Summary`
pub trait Summary: Debug {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // default implementation
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // Note implementing this will revert to the default implementation.
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub fn notify(item: &(impl Summary + Debug)) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_desugared<T: Summary + Debug>(item: &T) {
    notify(item);
}

pub fn notify_simplified<T>(item: &T)
where
    T: Summary + Debug,
{
    notify(item);
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Using Trait Bounds to Conditionally Implement Methods
// The standard library implements the ToString trait on any type that
// implements the Display trait.
//
// impl<T: Display> ToString for T {
// }

fn using_trait_bounds_to_conditionally_implement_methods() {
    let pair1 = Pair::new(12, 24);
    pair1.cmp_display();
}
