# Error Handling in Rust {#error-handling}

Rust provides a powerful error-handling system that helps ensure robustness and reliability. There are two main types of errors:

## 1. Unrecoverable Errors (`panic!`)

Unrecoverable errors occur when the program reaches an unexpected state and must terminate immediately. Rust handles these with `panic!()`.

### Example:
```rust
fn main() {
    panic!("Something went wrong!");
}
```

### When to Use `panic!()`
- **Critical failure** where recovery is impossible.
- **Development & debugging** to catch issues early.
- **Assertions** in tests using `assert!()` or `assert_eq!()`.

### Handling Panics Gracefully
Instead of crashing the whole program, you can use `std::panic::catch_unwind()` to handle a panic in certain cases:
```rust
use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        panic!("This will be caught");
    });
    
    if result.is_err() {
        println!("A panic occurred, but the program is still running.");
    }
}
```

## 2. Recoverable Errors (`Result<T, E>`)

Recoverable errors occur when an operation might fail but doesn't necessarily require termination. Rust uses the `Result<T, E>` type for these cases.

### Example:
```rust
use std::fs::File;
use std::io::Error;

fn main() -> Result<(), Error> {
    let file = File::open("config.txt")?; // If error, return it
    Ok(())
}
```

### `Result<T, E>` Explanation
- `Ok(T)`: The operation was successful and returns `T`.
- `Err(E)`: The operation failed and returns an error `E`.

### Common Ways to Handle `Result<T, E>`

#### 1. **Propagate the Error (`?` Operator)**
```rust
fn read_file() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("config.txt")?;
    Ok(content)
}
```
- The `?` operator **short-circuits** on `Err(E)`, returning it immediately.
- Only works in functions that return `Result<T, E>`.

#### 2. **Handle the Error Manually**
```rust
use std::fs::File;

fn main() {
    let file = File::open("config.txt");
    match file {
        Ok(f) => println!("File opened successfully!"),
        Err(e) => println!("Failed to open file: {}", e),
    }
}
```
- The `match` statement allows custom error handling.

#### 3. **Use `unwrap()` or `expect()`** *(Risky!)*
```rust
let file = File::open("config.txt").unwrap(); // Panics if it fails
let file = File::open("config.txt").expect("Failed to open file"); // Custom panic message
```
- Use **only if you are sure** the operation will succeed.
- Recommended for **quick prototyping** or **tests**.

#### 4. **Using `unwrap_or()` and `unwrap_or_else()`**
```rust
use std::fs::File;

fn main() {
    let file = File::open("config.txt").unwrap_or_else(|_| {
        println!("File not found, creating a new one.");
        File::create("config.txt").expect("Failed to create file")
    });
}
```
- `unwrap_or(default_value)` provides a fallback value.
- `unwrap_or_else(|err| handle_error(err))` allows custom error handling.

---

## 3. Custom Errors with `thiserror` and `anyhow`

For complex applications, defining custom error types is beneficial.

### Using `thiserror` for Custom Errors
```rust
use std::fs::File;
use std::io::{self, Read};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("File error: {0}")]
    FileError(std::io::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

fn read_file(file_name: &str) -> Result<String, MyError> {
    let mut file = File::open(file_name).map_err(MyError::FileError)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(MyError::FileError)?;
    Ok(contents)
}

fn main() {
    match read_file("example.txt") {
        Ok(contents) => println!("File content: {}", contents),
        Err(e) => println!("An error occurred: {}", e),
    }
}
```
- Use `#[error("message")]` to format error messages.
- Allows structured error handling.

### Using `anyhow` for Simpler Error Handling
```rust
use anyhow::{Context, Result};
use std::fs::File;

fn open_file() -> Result<File> {
    let file = File::open("config.txt").context("Could not open config file")?;
    Ok(file)
}
```
- `anyhow::Result<T>` allows returning **multiple error types** easily.
- `.context()` provides **custom error messages**.

---

## Summary of Rust Error Handling Techniques

| Technique | Use Case |
|-----------|---------|
| `panic!()` | Unrecoverable errors that should crash the program. |
| `Result<T, E>` | Recoverable errors where failure is an expected possibility. |
| `?` operator | Propagating errors in functions returning `Result<T, E>`. |
| `match` | Custom error handling logic. |
| `unwrap()` / `expect()` | Quick debugging, **but avoid in production**. |
| `unwrap_or()` / `unwrap_or_else()` | Provide default values or custom error handling. |
| `thiserror` | Custom structured error types. |
| `anyhow` | Simple error handling with better error messages. |

By following these practices, you can write **safe, reliable, and robust Rust applications**! 🚀
