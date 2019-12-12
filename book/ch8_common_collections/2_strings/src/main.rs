fn main() {
    print_strings(&[
        &String::from("السلام عليكم"),
        &String::from("Dobrý den"),
        &String::from("Hello"),
        &String::from("שָׁלוֹם"),
        &String::from("नमस्ते"),
        &String::from("こんにちは"),
        &String::from("안녕하세요"),
        &String::from("你好"),
        &String::from("Olá"),
        &String::from("Здравствуйте"),
        &String::from("Hola"),
        &"can also be defined like this".to_string(),
    ]);

    updating_a_string_with_push_str();
    updating_a_string_with_push();
    format_macro();
    inside_of_strings();
}

fn print_strings(s: &[&String]) {
    for (i, x) in s.iter().enumerate() {
        println!("String {}: {}", i, x);
    }
}

fn updating_a_string_with_push_str() {
    let mut s = "foo".to_string();
    s.push_str("bar");
    println!("s = {}", s);

    let mut s1 = String::from("whatever");
    let space = " ";
    let s2 = String::from("floats your boat");
    s1.push_str(space);
    s1.push_str(&s2);
    println!("s1 = {}", s1);
}

fn updating_a_string_with_push() {
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);
}

fn format_macro() {
    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

fn inside_of_strings() {
    let s = "नमस्ते";
    for (i, c) in s.chars().enumerate() {
        println!("Character {} of '{}' is '{}' and has {} bytes in UTF-8",
                 i, s, c, c.len_utf8());
    }
    for b in s.bytes() {
        println!("{}", b);
    }
}
