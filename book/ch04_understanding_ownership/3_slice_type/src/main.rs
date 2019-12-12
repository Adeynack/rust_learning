fn main() {
    let /*mut*/ sentence = String::from("This white wolf");
    println!(
        "First word of '{}' ends at index: {}",
        sentence,
        first_word_as_end_index(&sentence)
    );

    string_slices_basics();

    let first_word = first_word_slice(&sentence);

    // cannot borrow `sentence` as mutable because it is also borrowed as immutable
    // sentence.clear();

    println!("First word of '{}' using slices: {}", sentence, first_word);

    println!(
        "First word of '{}' using slices: {}",
        sentence,
        first_word_slice_param(&sentence) // the [..] seems to be optional
    );

    slice_of_whatever_type();
}

fn first_word_as_end_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slices_basics() {
    let s = String::from("Hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{} {}", hello, world);

    let hello2: &str = &s[..5]; // beginning to 5
    let world2: &str = &s[6..]; // 6 to end
    println!("{} {}", hello2, world2);

    let whole_string = &s[..];
    println!("{}", whole_string);

    let whole_string2 = &s[0..s.len()];
    println!("{}", whole_string2);

    // let multibyte_character = String::from("😡");
    // Next line crashes: byte index 1 is not a char boundary; it is inside '😡' (bytes 0..4) of `😡`
    // println!("{}", &multibyte_character[1..2]);
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice_param(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slice_of_whatever_type() {
    let a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    println!("Slice {:?} of {:?} is {:?}", a, [1..3], slice);
}
