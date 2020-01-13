use core::fmt;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Deref, DerefMut};

fn main() {
    placeholder_type_in_trait_definitions();
    default_generic_type_parameter_and_operator_overloading();
    fully_qualified_syntax_for_disambiguation();
    supertraits();
    newtype_pattern();
}

fn placeholder_type_in_trait_definitions() {
    // pub trait Iterator {
    //     type Item;
    //
    //     fn next(&mut self) -> Option<Self::Item>;
    // }

    struct Counter {
        counter: u32,
        max: u32,
    }

    impl Counter {
        fn to(max: u32) -> Counter {
            Counter {
                counter: 0,
                max,
            }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.counter == self.max {
                None
            } else {
                self.counter += 1;
                Some(self.counter)
            }
        }
    }

    let it = Counter::to(12);
    for x in it {
        print!("... {}", x);
    }
    println!(".")
}

fn default_generic_type_parameter_and_operator_overloading() {
    #[derive(Debug, PartialEq, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let a = Point { x: 1, y: 0 };
    let b = Point { x: 2, y: 3 };

    assert_eq!(a + b, Point { x: 3, y: 3 });
    println!("{} + {} = {}", a, b, a + b);

    #[derive(Debug, Copy, Clone)]
    struct Millimeters(u32);

    #[derive(Debug, Copy, Clone)]
    struct Meters(u32);

    impl Add for Millimeters {
        // by default: Add<Millimeters> because RHS=Self
        type Output = Millimeters;

        fn add(self, rhs: Millimeters) -> Self::Output {
            Millimeters(self.0 + rhs.0)
        }
    }

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + rhs.0 * 1000)
        }
    }

    let a = Millimeters(120);
    let b = Millimeters(4220);
    let c = Meters(2);

    println!("{:?} + {:?} = {:?}", a, b, a + b);
    println!("{:?} + {:?} = {:?}", a, c, a + c);
}

fn fully_qualified_syntax_for_disambiguation() {
    trait Pilot {
        fn fly(&self) -> &str;
    }

    trait Wizard {
        fn fly(&self) -> &str;
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) -> &str {
            "Eeeeeh ... This is your eeeeh ... captain speaking ..."
        }
    }

    impl Wizard for Human {
        fn fly(&self) -> &str {
            "Up!"
        }
    }

    impl Human {
        fn fly(&self) -> &str {
            "*waving arms furiously*"
        }
    }

    let person = Human;
    assert_eq!(person.fly(), "*waving arms furiously*");
    assert_eq!(Pilot::fly(&person), "Eeeeeh ... This is your eeeeh ... captain speaking ...");
    assert_eq!(Wizard::fly(&person), "Up!");

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            "Spot".to_string()
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            "puppy".to_string()
        }
    }

    assert_eq!(Dog::baby_name(), "Spot");
    assert_eq!(<Dog as Animal>::baby_name(), "puppy");
}

fn supertraits() {
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    let p = Point { x: 32, y: 48 };
    p.outline_print();
}

fn newtype_pattern() {
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    // allowing access to the immutable methods of the wrapped type
    impl Deref for Wrapper {
        type Target = Vec<String>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // allowing access to the mutable methods of the wrapped type
    impl DerefMut for Wrapper {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    let mut w = Wrapper(vec!["hello".to_string(), "world".to_string()]);
    println!("w = {}", &w);
    println!("w has a length of {}", w.len()); // this would not be possible without impl Deref

    w.push("this would not be possible without impl DerefMut".to_string());
    println!("w = {}", &w);
}
