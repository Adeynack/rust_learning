#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count of `a` after creating a = {}", Rc::strong_count(&a));
    println!("a = {:?}", a);

    let b = Cons(3, Rc::clone(&a));
    println!("count of `a` after creating b = {}", Rc::strong_count(&a));
    println!("b = {:?}", b);

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count of `a` after creating c = {}", Rc::strong_count(&a));
        println!("c = {:?}", c);
    }
    println!(
        "count of `a` after c goes out of scope = {}",
        Rc::strong_count(&a)
    );
}
