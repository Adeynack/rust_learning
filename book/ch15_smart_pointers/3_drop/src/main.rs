struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer `c` created with data `{}`", c.data);
    println!("CustomSmartPointer `d` created with data `{}`", d.data);

    drop(c);
    println!("CustomSmartPointer `c` dropped before the end of main.");
    // println!("c = `{}`", c.data);
    //                      ^^^^^^ value borrowed here after move
}
