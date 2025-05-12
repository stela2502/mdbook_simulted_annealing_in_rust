# Testing in Rust  {#testing}

Rust has a built-in testing framework that enables developers to write and run tests efficiently. Tests help ensure code correctness, prevent regressions, and improve maintainability.

## 1. Writing Basic Unit Tests

Rust tests are written inside a module annotated with `#[cfg(test)]` and individual tests use `#[test]`.

### Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }
}
```
- `#[cfg(test)]` ensures the module is only compiled when running tests.
- `#[test]` marks a function as a test.
- `assert_eq!(a, b)` checks if `a == b`, failing the test otherwise.

### Common Assertions:
| Macro | Description |
|--------|-------------|
| `assert!(condition)` | Fails if condition is false. |
| `assert_eq!(a, b)` | Fails if `a != b`, shows values. |
| `assert_ne!(a, b)` | Fails if `a == b`, shows values. |

## 2. Running Tests

To execute all tests, run:
```sh
cargo test
```

- Runs all test functions inside `#[cfg(test)]` modules.
- Captures standard output unless `--nocapture` is used:
  ```sh
  cargo test -- --nocapture
  ```
- Runs specific tests using:
  ```sh
  cargo test test_addition
  ```

## 3. Testing for Panics

Use `#[should_panic]` to test for expected panics.
```rust
#[test]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero() {
    let _ = 1 / 0;
}
```
- The test passes if it panics with the expected message.

## 4. Using `Result<T, E>` in Tests

Instead of panicking, tests can return `Result<(), E>` for better error handling.
```rust
#[test]
fn test_file_reading() -> Result<(), std::io::Error> {
    let content = std::fs::read_to_string("test_file.txt")?;
    assert!(content.contains("Hello"));
    Ok(())
}
```
- Recommended for tests involving I/O or other fallible operations.

## 5. Ignoring Tests

Some tests may take a long time. Use `#[ignore]` to exclude them from default runs:
```rust
#[test]
#[ignore]
fn long_running_test() {
    std::thread::sleep(std::time::Duration::from_secs(10));
}
```
Run ignored tests with:
```sh
cargo test -- --ignored
```

## 6. Benchmarking with `#[bench]`

Rust provides benchmarking via the `test` crate (nightly only):
```rust
#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn bench_addition(b: &mut Bencher) {
    b.iter(|| 2 + 2);
}
```
Run benchmarks with:
```sh
cargo bench
```

## 7. Integration Tests

Integration tests are placed in the `tests/` directory and test the external interface of the library.

### Example `tests/integration_test.rs`:
```rust
use my_crate::add;

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```
- Uses `use my_crate::*;` to import public API functions.
- Each file in `tests/` is compiled separately.

## 8. Testing the Executable

For projects that generate a binary, you can test the executable by running it as a subprocess and checking its output.

### Example `tests/test_binary.rs`:
```rust
use std::process::Command;

#[test]
fn test_executable_output() {
    let output = Command::new("target/debug/my_binary")
        .arg("--version")
        .output()
        .expect("Failed to execute binary");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("my_binary 1.0.0"));
}
```
- Uses `std::process::Command` to execute the compiled binary.
- Checks if the output matches the expected version string.

## Summary

| Feature | Description |
|---------|-------------|
| `#[test]` | Defines a unit test. |
| `assert!`, `assert_eq!`, `assert_ne!` | Assertion macros. |
| `cargo test` | Runs all tests. |
| `#[should_panic]` | Tests expected panics. |
| `Result<(), E>` | Allows fallible tests. |
| `#[ignore]` | Skips tests unless explicitly run. |
| `cargo bench` | Runs performance benchmarks (nightly only). |
| Integration Tests | Stored in `tests/`, test public APIs. |
| Executable Testing | Uses `std::process::Command` to verify binary output. |

By following these testing best practices, you can ensure your Rust code is robust, maintainable, and reliable! 🚀


