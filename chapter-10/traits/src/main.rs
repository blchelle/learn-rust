use std::fmt::Display;

// This is a simple trait with a single message signature that does not
// provide any default implementation
pub trait Summary {
    fn summarize(&self) -> String;
}

// But traits can also provide default implementations for some, none, or all
// of their required methods. So really anything can have the trait
// SummarizeDefault, though it has to be specified explicitly unlike go
pub trait SummaryDefault {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more)...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// NewsArticle explicitly has the SummaryDefault trait. It doesn't have to
// provide an implementation for summarize (though it can), because it already
// has a default implementation
impl SummaryDefault for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("The authors name is: {}", self.author)
    }
}

// Tweet implements the summarize method so it has the Summary trait
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/**
 * An interesting thing here is the &impl in from of the Summary
 * It's kinda like its saying that item is a reference to an implementation
 * of Summary
 */
pub fn notify(item: &impl Summary) {
    println!("Breaking news!: {}", item.summarize());
}

/**
 * This function definition and the previous one are completely equivalent,
 * the previous one is just a little bit of syntactic sugar, though I don't
 * have any problems with this syntax. I think its a little more clear
 */
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news!: {}", item.summarize());
}

/**
 * This function definition shows how to have a type parameter that has
 * multiple traits simultaneously
 */
pub fn multiple_traits(item: &(impl Summary + Display)) {
    println!("Breaking news!: {}", item.summarize());
}

/**
 * Too many trait bounds can make your function signatures hard to read,
 * a solution to this is to use the 'where' clause which separates the
 * function signature from the trait bound. Making it easier to read
 */
pub fn where_clause<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Copy,
{
}

/**
 * We can also return traits from functions, this is especially useful for
 * closures and iterators
 */
pub fn returns_trait() -> impl Summary {
    Tweet {
        username: String::from("lalaloo_thoughts"),
        content: String::from("Hot dog is NOT a sandwich"),
        reply: false,
        retweet: false,
    }
}

/**
 * Oddly enough though, we cannot return multiple concrete types from the same
 * function that returns impl Trait, even if both types implement the trait
 */
// pub fn returns_trait_bad(switch: bool) -> impl Summary {
//     if switch {
//         Tweet {
//             username: String::from("lalaloo_thoughts"),
//             content: String::from("Hot dog is NOT a sandwich"),
//             reply: false,
//             retweet: false,
//         }
//     } else {
//         NewsArticle {
//             author: String::from("Brock Chelle"),
//             content: String::from("Hot dog IS a sandwich"),
//             headline: String::from("Is hot dog a sandwich?"),
//             location: String::from("Edmonton"),
//         }
//     }
// }

fn main() {
    // Creates a tweet and summarizes it using the custom implementation
    let tweet = Tweet {
        username: String::from("lalaloo_thoughts"),
        content: String::from("Hot dog is NOT a sandwich"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let news = NewsArticle {
        author: String::from("Brock Chelle"),
        content: String::from("Hot dog IS a sandwich"),
        headline: String::from("Is hot dog a sandwich?"),
        location: String::from("Edmonton"),
    };
    println!("Breaking News {}", news.summarize());
    println!("{}", news.summarize_author());

    // The line below wouldn't compile because NewsArticle doesn't implement
    // the Display trait
    // println!("{}", multiple_traits(news))
}
