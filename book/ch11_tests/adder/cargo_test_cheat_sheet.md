Cargo Test Cheat Sheet
===

## Running Tests in Parallel or Consecutively

Do not run tests in parallel, but limiting the number of parallel runners to 1.

```bash
cargo test -- --test-thread=1
```

## Showing Function Output

By default, output will be captured by the test context and not displayed
at runtime unless the test fails.

We can prevent that behaviour and let the output occur as it is executed.

```bash
cargo test -- --nocapture
```

## Running a single test

We can run just the one test called `one_hundred`.

```bash
cargo test one_hundred
```
Note: We can’t specify the names of multiple tests in
this way; only the first value given to `cargo test` will
be used. But there is a way to run multiple tests.

## Filtering to run multiple tests

We can run every test whose name starts with `add`.

```
$ cargo test add
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok
```

## Ignore tests

```rust
#[test]
#[ignore]
fn expensive_test() {
    // ...
}
```

We can than, on demand, run ignored tests.

```bash
cargo test -- --ignored
```

## Just run a specific integration test

```bash
cargo test --test integration_test
```

Just the test file `tests/integration_test.rs` will be executed.

