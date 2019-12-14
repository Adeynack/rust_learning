// Files in subdirectories of the tests directory don’t get
// compiled as separate crates or have sections in the test output.
//
// Being in the `tests/common` folder, this will not be executed
// as a test crate.
//

pub fn setup() {
    println!("Setting stuff up for integration tests")
}
