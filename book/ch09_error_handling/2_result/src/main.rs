use std::fs::File;
use std::io::{Read, ErrorKind};
use std::{io, fs};
use std::error::Error;

// to use the ? operator inside of the `main`, it needs to return a `Result`.
fn main() -> Result<(), Box<dyn Error>> {

    // read_or_panic("hello.txt"); // panics
    read_or_panic("src/main.rs");

    open_or_create_or_panic("src/main.rs");

    // println!("{:?}", File::open("hello.txt").unwrap()); // panics
    // println!("{:?}", File::open("hello.txt").expect("Failed to open hello.txt")); // panics

    println!("{:?}", read_or_propagate("src/main.rs"));
    println!("{:?}", read_or_propagate_shorter("src/main.rs"));
    println!("{:?}", fs::read_to_string("src/main.rs").map(|c| c[0..64].to_string()));

    let f = File::open("hello.txt")?;
    println!("{:?}", f);
    Ok(())
}

fn read_or_panic(file_path: &str) {
    let f = File::open(file_path);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("File {} could not be found", file_path),
            other_error => panic!("Problem of kind {:?} opening the file: {:?}", other_error, error),
        },
    };

    let mut content = String::new();
    match f.read_to_string(&mut content) {
        Ok(read_size) => println!("Read {} bytes", read_size),
        Err(error) => panic!("Unable to read the content of the file: {:?}", error)
    }
    println!("{}", content[0..64].to_string());
}

fn open_or_create_or_panic(file_path: &str) {
    let f = File::open(file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", f);
}

fn read_or_propagate(file_path: &str) -> Result<String, io::Error> {
    let mut f = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut content = String::new();
    match f.read_to_string(&mut content) {
        Ok(_) => Ok(content[0..64].to_string()),
        Err(error) => Err(error),
    }
}

fn read_or_propagate_shorter(file_path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file_path)?.read_to_string(&mut s)?;
    Ok(s[0..64].to_string())
}
