#[derive(Debug)]
struct Person {
    first_name: String,
}

fn main() {
    //
    // Building vectors
    //

    let v1: Vec<i32> = Vec::new();
    let v2 = Vec::<i32>::new();
    let v3 = vec![1, 2, 3];

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
    println!("v3 = {:?}", v3);

    // v1.push(42); // cannot work, since v1 is not mutable

    let mut v4 = vec![1, 2, 3];
    println!("v4 before push = {:?}", v4);
    v4.push(4);
    println!("v4 after push = {:?}", v4);

    //
    // Dropping a Vector Drops Its Elements
    //

    let dave = Person { first_name: String::from("Dave") };
    let people_moved = vec![dave]; // value moved here
    // println!("{:?}", dave); // value borrowed here after move
    println!("{:?}", people_moved);

    let nancy = Person { first_name: String::from("Nancy") };
    let people_borrowed: Vec<&Person> = vec![&nancy];
    println!("{:?}", nancy); // was borrowed, not moved; so can still be used afterwards
    println!("{:?}", people_borrowed);

    //
    // Reading Elements
    //

    let tim = Person { first_name: String::from("Tim") };
    let people = vec![&tim, &nancy];

    // with index
    println!("Tim is {:?}", people[0]);
    println!("Nancy is {:?}", people[1]);
    // println!("This would crash {:?}", people[2]); // index out of bounds: the len is 2 but the index is 2

    // with `get`
    match people.get(1) {
        Some(at_index_1) => println!("At index 1, there is {:?}", at_index_1),
        None => println!("There is nothing at index 1"),
    }
    match people.get(2) {
        Some(at_index_1) => println!("At index 2, there is {:?}", at_index_1),
        None => println!("There is nothing at index 2"),
    }

    // using the index method would move the ownership if not used with &
    let bob = Person { first_name: String::from("Bob") };
    let owning_vector = vec![bob];
    // let bob_from_vec = owning_vector[0]; // this ownership move is not allowed
    let bob_from_vec = &owning_vector[0]; // this is ok because borrowed from the vector
    println!("Bob is {:?}", bob_from_vec);
    match owning_vector.get(0) {
        Some(at_index_0) => println!("At index 2, there is {:?}", at_index_0),
        None => println!("There is nothing at index 2"),
    }

    //
    // Iterating Values
    //

    let immut_vec = vec![100, 32, 57];
    for i in &immut_vec {
        println!("{}", i);
    }

    let mut mut_vec = immut_vec.clone();
    for i in &mut mut_vec {
        *i += 50;
    }
    println!("{:?}, after mutation: {:?}", immut_vec, mut_vec);

    //
    // Using an Enum to Store Multiple Types
    //

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
