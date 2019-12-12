fn main() {
    references_and_borrowing();
    mutable_references();
}

fn references_and_borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// usize calculate_length(std::string &s)
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_references() {
    let mut s = String::from("hello");
    change(&mut s);

    println!("Mutated s = {}", s);

    let s1 = &mut s;
    // let s2: &mut String = &mut s; // second mutable borrow occurs here, will not compile.
    println!("Mutated s = {}", s1);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
