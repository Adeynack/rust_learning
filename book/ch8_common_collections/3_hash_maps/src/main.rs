use std::collections::HashMap;

//struct Person {
//    id: i128,
//    first_name: String,
//    last_name: String,
//}

fn main() {
    creating_hash_map();
    collect_from_vector();
    accessing_values();
    iterate_on_a_hash_map();
    updating_a_hash_map();
}

fn creating_hash_map() {
    // Type of key and value can be implicit-ed by the first call to `insert`.
    let mut scores1 = HashMap::new();

    scores1.insert("Blue".to_string(), 10);
    scores1.insert("Yellow".to_string(), 50);
    println!("Scores 1: {:#?}", scores1);

    // Type of key and value can be explicit.
    let mut scores2 = HashMap::<String, i128>::new();
    scores2.insert("Blue".to_string(), 10);
    scores2.insert("Yellow".to_string(), 50);
    println!("Scores 2: {:#?}", scores2);
}

fn collect_from_vector() {
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("Collected scores: {:#?}", scores);
}

fn accessing_values() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Score: {:?}", score);
    let non_existing_score = scores.get("foo");
    println!("Score: {:?}", non_existing_score);
}

fn iterate_on_a_hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn updating_a_hash_map() {
    let mut scores = HashMap::new();

    // insert will add new values or replace existing

    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);

    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // entry/or_insert will add new values and not replace existing

    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // using entry/or_insert to update based on existing value (or default).
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word count: {:#?}", word_count);
}
