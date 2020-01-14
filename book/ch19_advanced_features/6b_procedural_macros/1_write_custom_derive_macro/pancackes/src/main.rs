use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Bleh;

fn main() {
    Pancakes::hello_macro();
    Bleh::hello_macro();
}
