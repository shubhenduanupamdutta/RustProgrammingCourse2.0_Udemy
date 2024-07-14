# Testing Code
---------------------------------------------------------------

## Unit Testing
- #### When you create a library crate using `cargo new <crate-name> --lib`, it will automatically generate a `lib.rs` file in the `src` directory.
- #### This `lib.rs` file will contain a sample function and test for that function.

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```
- We see a module called `tests`, with a configuration attribute `#[cfg(test)]`.
- This configuration attribute `#[cfg(test)]` tells the compiler to compile and run the code only when running tests. This stops the test code from being compiled into the final binary, during normal compilation.
- The `tests` module contains a function called `it_works`, which tests the `add` function.
- Test functions are marked with the `#[test]` attribute.
- The `assert_eq!` macro is used to compare the result of the `add` function with the expected value.
- If `assert_eq!` fails, the macro will panic and the test will fail.
- `panic` refers to a situation where the program encounters an error and cannot continue execution. It will print an error message and exit the program.
- When `panic` occurs normal execution of the program stops and the program exits.
- We can also use `assert!` macro to check if a condition is true, if the result expression is a boolean.
```rust
assert_eq!(result, 4);
assert!(result > 0);
```
- To run the tests, we can use the `cargo test` command.
- `assert_ne!` macro is used to check if two values are not equal.
```rust
assert_ne!(result, 5);
```
- Test function will pass if the last assert statement is true, otherwise it will fail.
- Test function can also return a `Result` type. This is typically used when the function being tested return a `Result` type.
```rust
#[test]
fn add_positive_numbers(num1: i32, num2: i32) -> Result<i32, String> {
    if num1 < 0 || num2 < 0 {
        return Err("Both numbers must be positive".to_string());
    }
    Ok(num1 + num2)
}

[#cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() -> Result<(), String> {
        let some_result = add_positive_numbers(2, -1)?;
        Ok(())
    }
}
```
- In the above example, the `add_positive_numbers` function returns a `Result` type.
- The test function `test_add_positive_numbers` also returns a `Result` type.
- Instead of using `assert_eq!`, we use the `?` operator to propagate the error if the function returns an `Err` value.
- If the function returns an `Ok` value, the test function will return `Ok(())`.

----------------------------------------------------------------
## Unit Testing (Testing Panics)
#### - We can also test if a function panics when it is supposed to panic. This will not work for functions that return Result type.
```rust
pub fn new_2(radius: f32) -> Circle {
        match radius {
            ..=0.0 => panic!("Radius cannot be zero or negative"),
            _ => Circle { radius },
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Radius cannot be zero or negative")]
    fn test_new_2() {
        new_2(0.0);
    }
}
```
- We can also use the `#[should_panic]` attribute to test if a function panics.
- The `#[should_panic]` attribute takes an optional `expected` parameter, which is the error message that the function is expected to panic with. If the function panics with a different error message, the test will fail.

### Side Notes
- **Test function can be standalone, meaning that it is not necessary for them to be included in the test module.**
- **Convention is to define a test module, which contains your test functions.**
- **Functions inside the test module which don't have the `#[test]` attribute will not be run as tests. They are treated as helper functions. They are used by test functions to avoid code duplication, fulfill some preconditions, etc.**
- **Rusts test framework allows tests to access private functions as long as tests are in the same module.**
- Testing private function is a topic of debate in the Rust community. Some people argue that tests should only test the public API of the library, while others argue that testing private functions is also important. The decision is up to you.

---------------------------------------------------------------
## Controlling Test Execution

### Choosing the crate to test
- If we have multiple binary crates in our project, then `cargo test` will run tests for all the binary crates, and create separate reports for each binary crate.
- To view only the test results for a specific binary crate, we can use the `--bin` flag.
```shell
cargo test --bin <binary-crate-name>
```
- To view test reports from only library crates, we can use the `--lib` flag.
```shell
cargo test --lib
```

### Enabling Output
- By default, `cargo test` when run without any flags, will run tests for all binary and library crates, and do not generate any outputs. Even if there are print statements in the test functions, they will not be printed.
- To view the output of the test functions, we can use the `-- --show-output` flag.
```shell
cargo test -- --show-output
```

### Running Specific Tests
- To run only a specific test function, we can use the name of the test function
```shell
cargo test --lib <test_function_name>
```
- We can also filter out the test function by using the substring of the test function name.
```shell
cargo test --lib <substring>
```
- For example, in this project, we can run the test function `should_not_create_circle` and `should_not_create_and_panic` by using the following command.
```shell
cargo test --lib should_not
```
- This will filter out all the test that contains the substring `should_not`.

### Ignoring Tests
- Some times you may want to ignore some tests while executing others. It may be because the tests are platform specific, or they are slow, or they are not relevant for the current development cycle.
- We can use the `#[ignore]` attribute to ignore a test function.
```rust
#[test]
#[ignore]
fn test_add_positive_numbers() -> Result<(), String> {
    let some_result = add_positive_numbers(2, -1)?;
    Ok(())
}
```
- By default `cargo test` will not run the ignored tests.
- To run the ignored tests, we can use the `-- --ignored` flag.
```shell
cargo test -- --ignored
```

---------------------------------------------------------------
## Integration Tests
#### - Integration tests are used to test the interaction between different modules of the library crate.
#### - Integration tests are placed in the separate `tests` directory, and are only supposed to test the public interface.
#### - Integration tests are used to verify that unit of codes work together as expected.
#### - Cargo knows to look for integration tests in the top level `tests` directory.

```rust
use testing_code::{Category, Customer, Order, Product};

#[test]
fn test_total_bill_without_discount() {
    let product = Product::new(1, "Harry Potter Book 1".to_string(), 100.0, Category::Books);

    let customer = Customer::new(1, "Bob".to_string(), "bob_mumbai@gmail.com".to_string());

    let order = Order::new(2, product, customer, 2);

    assert_eq!(format!("{:.2}", order.total_bill()), "220.00");
}

#[test]
fn test_total_bill_with_discount() {
    let product = Product::new(1, "Harry Potter Book 1".to_string(), 100.0, Category::Books);

    let customer = Customer::new(1, "Bob".to_string(), "bob_mumbai@gmail.com".to_string());

    let order = Order::new(2, product, customer, 6);

    assert_eq!(format!("{:.2}", order.total_bill()), "594.00");
}
```
- Above code is written in `order_test.rs` file in the `tests` directory.
- We are testing the `total_bill` method of the `Order` struct.
- When we run `cargo test`, cargo will look for the `tests` directory and run all the integration tests in that directory, including all the other tests.
- If you want to only run the test in the `order_test.rs` file, you can use the following command.
```shell
cargo test --test order_test
```
- Every file in the `tests` directory is treated as a separate crate and considered as a test file by cargo.
- Sometimes we may want to create separate crates for isolation and organization purposes. In such cases, we can create a separate directory for the integration tests and use the `--test` flag to run the tests in that directory.
- There can also be a downside, it may happen that some code, which setups the test, need to be shared between the integration tests.
- In such cases, we can create a folder, and use `mod.rs` inside it to share the setup code between the integration tests.
- **Integration test can not import items from binary crate directly.**
- **That's why common pattern in Rust is to have a small binary crate and move all the functionality to the library crate.**
---------------------------------------------------
## Benchmarking
- Performance is important for many rust programs. 
- Following are the few techniques which can improve the performance of the code.
- Some techniques are entirely rust specific, while others can, with some modification, applied to other languages as well.

- **Benchmarking:** Benchmarking is the process of measuring the performance of the code. It is used to identify the bottlenecks in the code and optimize them. It is also used to compare two code snippets which do the same thing.
    - Sometime it involves comparing two different implementation
    - Other times, it involves comparing two versions of same implementation.
- In benchmarking different factors may be taken into considerations, such as
    - Processing Time
    - Memory Usage
    - Disk Access time etc.

- In this section, we will focus on benchmarking the processing time of the code.
- I have written two sorting algorithm in `sorting.rs` and imported them in `lib.rs` file.
```rust
pub fn sort_algo_1<T: PartialOrd>(arr: &mut Vec<T>) {
    let mut swapped: bool = false;
    for i in 0..(arr.len() - 1) {
        if arr[i] > arr[i + 1] {
            arr.swap(i, i + 1);
            swapped = true;
        }
    }
    if swapped {
        sort_algo_1(arr);
    }
}

pub fn sort_algo_2<T: Ord>(arr: &mut Vec<T>) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}
```
- Let's see how we can benchmark these two sorting algorithms.
- There is a built in support for benchmarking in Rust, but it is unstable and only available in nightly version.
- We can use the `criterion` crate to benchmark the code.
---------------------------------------------------
### Using Criterion Crate
- Add the following dependency in the `Cargo.toml` file.
```toml
[dev-dependencies]
criterion = "0.4.0"

[[bench]]
name = "sorting_benchmark"
harness = false # Disable the default benchmark harness
```
- Create a new folder called `benches` in the root directory.
- Create a new file called `sorting_benchmark.rs` in the `benches` directory.
- Name of the file should be same as the name of the benchmark in the `Cargo.toml` file.
```rust
use testing_code::sorting::{sort_algo_1, sort_algo_2};

use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers = vec![
        12, 47, 23, 9, 5, 30, 2, 19, 45, 6, 14, 33, 79, 26, 53, 8, 67, 21, 10, 4,
    ];
    c.bench_function("Sorting Algorithm", |b| {
        b.iter(|| sort_algo_1(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);
```
- In the above code, we are benchmarking the `sort_algo_1` function.
- To create a new benchmark, we will call a benchmark function `c.bench_function` and pass the name of the benchmark and the function to be benchmarked.
- Sort array function will be run many times to get accurate and robust results.
- Next we will use `criterion_group!` and `criterion_main!` macros to run the benchmark.
- `criterion_group!` macro is used to group multiple benchmarks together, with same configuration.
- `criterion_main!` macro is used to run the benchmarks.
- To run the benchmark, we can use the following command.
```shell
cargo bench
```

- Our results show that performance has regressed for algorithm 2.  