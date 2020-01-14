// See for more: https://danielkeep.github.io/tlborm/book/index.html

fn main() {
    declarative_macros();
}

#[macro_export]
macro_rules! new_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn declarative_macros() {
    let v = new_vec![1, 2, 3, 4];
    println!("{:?}", v);
}
