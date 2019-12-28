use std::ops::Deref;

fn main() {
    let x = 5;

    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

struct MyBox<T>(T);

impl<T> MyBox<T>
where
    T: std::fmt::Debug,
{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
where
    T: std::fmt::Debug,
{
    type Target = T;

    fn deref(&self) -> &T {
        println!("Derefencing MyBox({:?})", self.0);
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
