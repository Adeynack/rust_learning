fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(_f: Thunk) {}

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("from the function"))
    }

    takes_long_type(f);
    let _f2 = returns_long_type();
}
