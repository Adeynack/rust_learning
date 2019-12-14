use std::fmt::Display;

fn main() {
    basic_lifetime_usage();
    shortest_lifetime_usage();
    use_lifetime_in_struct();
    use_first_word();
    use_important_excerpt_methods_with_lifetime();
    usage_of_longest_with_an_annoucement();
}

// fn fails_to_compile_value_does_not_live_long_enough() {
//     let r;
//
//     {
//         let x = 5;
//         r = &x;
//         ^^^^^^ borrowed value does not live long enough
//     }
//     - `x` dropped here while still borrowed
//
//     println!("r: {}", r);
//                       - borrow later used here
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn basic_lifetime_usage() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("The longest string is {}", result);
}

fn shortest_lifetime_usage() {
    let s1 = "long string is long".to_string();

    {
        let s2 = "xyz".to_string();
        let result = longest(s1.as_str(), s2.as_str());
        println!("The longest string is {}", result);
    }
}

// fn invalid_lifetime_usage() {
//     let s1 = "long string is long".to_string();
//     let result;
//     {
//         let s2 = "xyz".to_string();
//         result = longest(s1.as_str(), s2.as_str());
//         //                            ^^ borrowed value does not live long enough
//     }
//     println!("The longest string is {}", result);
//     //                                   ------ borrow later used here
// }

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn use_lifetime_in_struct() {
    let novel = "Call me Ishmael. Some years ago...".to_string();
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("First sentence: {:?}", i);
}

// example of Lifetime Elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn use_first_word() {
    let s = "Foo Bar Bleh".to_string();
    println!("First word: {}", first_word(s.as_str()));
}

// Lifetimes in methods

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn use_important_excerpt_methods_with_lifetime() {
    let ie = ImportantExcerpt { part: "asd" };
    println!("level = {}", ie.level());
    ie.announce_and_return_part("Oyee Oyee!");
}

fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Annoucement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn usage_of_longest_with_an_annoucement() {
    let s1 = "foo".to_string();
    let s2 = "bleh".to_string();
    println!(
        "Longest, after announcement, is: {}",
        longest_with_an_annoucement(s1.as_str(), s2.as_str(), "Ah ben, taaa!")
    );
}
