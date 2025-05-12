# Variables - Strict Types and Ownership in Rust

## Strict Types in Rust

Rust is a statically typed language, meaning you **must define the types** of variables and data structures explicitly. This ensures **type safety** at **compile time**.

### Example 1: Strict Typing in Rust

Let's start by defining variables with specific types:

```rust
fn main() {
    let integer: i32 = 10;  // 32-bit signed integer
    let floating_point: f64 = 10.5;  // 64-bit floating-point number
    let boolean: bool = true;  // Boolean value

    println!("Integer: {}, Floating-point: {}, Boolean: {}", integer, floating_point, boolean);
}
```

This does work as expected, but what happens if we set the boolean to 1 instead?

```rust
fn main() {
    let integer: i32 = 10;  // 32-bit signed integer
    let floating_point: f64 = 10.5;  // 64-bit floating-point number
    let boolean: bool = 1;  // Boolean value

    println!("Integer: {}, Floating-point: {}, Boolean: {}", integer, floating_point, boolean);
}
```

#### What happens here?
- Rust expects each variable to be of the specified type (`i32`, `f64`, `bool`).
- If you try to assign a value with an incompatible type (e.g., `1` to an `bool`), the **compiler will throw an error**.

```
error[E0308]: mismatched types
 --> src\main.rs:8:9
  |
4 |     let boolean: bool = 1;
  |         ^^^^^^^^^^^^^^ expected 'bool', found floating-point number
```

This strict checking ensures type safety and helps avoid runtime errors related to type mismatches.

#### Key Points:
- **Rust enforces type correctness at compile time**. You **cannot implicitly convert between types** (like R or Python might allow).
- **Explicit type annotations** help prevent unintended errors and increase code clarity.

---

## Ownership in Rust

One of Rust's most important and unique features is its **ownership model**. It ensures **memory safety** by enforcing strict rules about **who owns data** and when data is **dropped (freed)**.

### Ownership Rules in Rust:
1. **Each value in Rust has a single owner**.
2. **When the owner goes out of scope**, Rust **automatically frees the memory**.
3. **You cannot have more than one owner** at a time. If you want to **transfer ownership**, you use **borrowing** or **cloning**.

### Example 2: Ownership in Rust

Let's look at a simple example that demonstrates ownership.

```rust
fn main() {
    let s1 = String::from("Hello, Rust!");  // s1 owns the String

    let s2 = s1;  // Ownership of the String is moved from s1 to s2
    println!("{}", s2);  // This works, since s2 is the owner now

    // After s2 goes out of scope, the memory will be freed.
}
```

This version simply prints the values from s2 - but would s1 still be usable?

```rust
fn main() {
    let s1 = String::from("Hello, Rust!");  // s1 owns the String

    let s2 = s1;  // Ownership of the String is moved from s1 to s2
    println!("{}", s1);  // This works, since s2 is the owner now

    // After s2 goes out of scope, the memory will be freed.
}
```
   let s2 = s1.clone();  // Ownership of the String is moved from s1 to s2
  |      
#### Explanation:
- `s1` is the **owner** of the `String` object.
- When `s1` is assigned to `s2`, **ownership** of the `String` is **moved** from `s1` to `s2`.
- After the move, `s1` is no longer valid, and trying to use it (e.g., `println!("{}", s1)`) will result in a **compiler error**:  
  ```
  error[E0382]: use of moved value: `s1`
   --> src\main.rs:7:22
    |
  7 |     println!("{}", s1);  // ERROR: value moved, cannot use
    |                      ^^^ value moved here
  ```

This behavior is critical because it ensures that **Rust doesn’t accidentally create multiple owners** of the same data (which could lead to **data races or memory leaks**).

And you also see that the Rust compiler directly tries to give you a hint in how to circumvent this problem:

```
help: consider cloning the value if the performance cost is acceptable
  |
4 |   let s2 = s1.clone();  // Ownership of the String is moved from s1 to s2
  |     
```

### Example 3: Borrowing in Rust
Rust allows you to **borrow** data, either immutably or mutably.

```rust
fn main() {
    let s1 = String::from("Hello, Rust!");  // s1 owns the String

    // Immutable Borrowing
    let s2 = &s1;  // s2 borrows s1 immutably
    println!("{}", s2);  // This works because s2 is just borrowing s1

    // Mutable Borrowing
    let mut s3 = String::from("Mutable borrow!");
    let s4 = &mut s3;  // s4 borrows s3 mutably
    s4.push_str(" Now it’s mutable!");
    println!("{}", s4);  // Prints: Mutable borrow! Now it’s mutable!
    
    // You can't have both mutable and immutable borrows at the same time:
    let s5 = &s1;  // ERROR: cannot borrow `s1` as immutable because it's already borrowed as mutable
}
```

#### Explanation:
- **Immutable borrowing** (`let s2 = &s1`) allows multiple references to the same data but **doesn’t allow modification**.
- **Mutable borrowing** (`let s4 = &mut s3`) gives exclusive access to the data, and **no other borrows** (immutable or mutable) can exist while it’s borrowed mutably.
- The rules prevent **data races** by ensuring no one can have mutable access while others are reading (or mutably accessing the data).

---

## Combining Types and Ownership

Let's combine both **strict typing** and **ownership** in a small example where we use both concepts together.

```rust
fn main() {
    let num: i32 = 42;  // i32 type (strict typing)
    let num_copy = num;  // Ownership of num is copied to num_copy (as num is Copy type)

    println!("num: {}, num_copy: {}", num, num_copy);  // Both can still be used

    let s1 = String::from("Rust Ownership!");  // String type (non-Copy type)
    let s2 = s1;  // Ownership of s1 is moved to s2
    // println!("{}", s1);  // ERROR: s1 is no longer valid
    println!("{}", s2);  // This works, since s2 now owns the String
}
```

### Output:
```
num: 42, num_copy: 42
Rust Ownership!
```

- **Copy types** (like `i32`) do not move ownership. They **duplicate the data** when assigned to another variable.
- **Non-copy types** (like `String`) **move ownership** when assigned to another variable, which makes the first variable invalid.

---

## Summary of Ownership and Strict Types:

- **Rust’s strict typing** ensures type safety at compile time, preventing mismatched types and logic errors that would only be caught at runtime in languages like R or Python.
- **Rust’s ownership model** prevents issues like double freeing of memory and data races, making it one of the **safest systems languages** available. The rules around ownership, borrowing, and moving data are **central to memory management** in Rust, giving developers fine-grained control over memory safety without a garbage collector.

Even so will not face these problems today it's extremely important to know about this before you try to program in Rust.
