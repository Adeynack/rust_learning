use std::cell::RefCell;
use std::rc::Rc;

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

impl List {
    fn from_ref(first_element: &Rc<RefCell<i32>>) -> List {
        Cons(Rc::clone(&first_element),
             Rc::new(Nil))
    }

    fn from_i32_and_ref(left: i32, right: &Rc<List>) -> List {
        Cons(Rc::new(RefCell::new(left)),
             Rc::clone(&right))
    }
}

fn main() {
    let value = Rc::new(RefCell::new(5));

//    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let a = Rc::new(List::from_ref(&value));
//    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let b = List::from_i32_and_ref(6, &a);
//    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    let c = List::from_i32_and_ref(10, &a);

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
