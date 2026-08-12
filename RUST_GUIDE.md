# Rust Programming Guide

A comprehensive guide to core Rust concepts with examples.

---

## 1. Variables

Variables are immutable by default. Use `mut` for mutability. Constants use `const`.

```rust
let x = 5;              // immutable (cannot change)
let mut y = 10;         // mutable (can change)
y = 15;                 // this works
const MAX: u32 = 100;   // constant — type required, never changes
```

**Variable Shadowing** — declare a new variable with the same name:
```rust
let x = 5;
let x = x + 1;  // shadows previous x (16)
```

---

## 2. Functions

Functions start with `fn`, use snake_case, and have typed params/returns.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // implicit return (no semicolon)
}

fn greet(name: &str) {
    println!("Hello, {}", name);
}
```

---

## 3. Conditionals

Use `if`, `else if`, `else`.

```rust
if age >= 18 {
    println!("Adult");
} else if age >= 13 {
    println!("Teen");
} else {
    println!("Child");
}
```

Also usable as an expression:
```rust
let result = if age >= 18 { "adult" } else { "minor" };
```

---

## 4. Loops

Three types: `loop`, `while`, `for`.

- **`loop`** — infinite until `break`
- **`while condition`** — runs while condition is true
- **`for .. in`** — iterate over a range or collection

```rust
// infinite loop with break
let mut counter = 0;
loop {
    counter += 1;
    if counter == 5 { break; }
}

// while loop
while counter < 10 { counter += 1; }

// for loop
for i in 0..5 { println!("{}", i); }  // 0 to 4
```

---

## 5. Structs

Structs group related data together.

```rust
struct User {
    name: String,
    age: u32,
    active: bool,
}

let user = User {
    name: String::from("sudhir"),
    age: 25,
    active: true,
};
println!("{}", user.name);
```

**Tuple struct** (no named fields):
```rust
struct Color(u8, u8, u8);
let black = Color(0, 0, 0);
```

---

## 6. Enums

Enums define a type that can be one of several variants.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Shape {
    Circle(f64),        // variant with data
    Rectangle(f64, f64),
    Triangle(u32),
}

let go = Direction::Left;
let shape = Shape::Circle(3.5);
```

---

## 7. Option<T>

Represents an optional value — something or nothing. `None` or `Some(T)`.

```rust
let some_number: Option<u8> = Some(5);
let no_number: Option<u8> = None;

// Safe pattern matching
match some_number {
    Some(n) => println!("Got {}", n),
    None => println!("No value"),
}

// Unwrap or default
let x = some_number.unwrap_or(0);  // 5
```

---

## 8. Result<T, E>

Represents success (`Ok`) or failure (`Err`).

```rust
use std::fs::File;

fn open_file(path: &str) -> Result<File, std::io::Error> {
    File::open(path)  // returns Ok(file) or Err(error)
}

match open_file("hello.txt") {
    Ok(file) => println!("Opened!"),
    Err(e) => println!("Error: {}", e),
}

// Using unwrap_or
let f = open_file("hello.txt").unwrap_or_else(|_| panic!("Cannot open"));
```

---

## 9. Pattern Matching

Use `match` to compare a value against patterns.

```rust
let number = 13;

match number {
    1 => println!("One"),
    2..=12 => println!("Two to twelve"),
    _ => println!("Something else"),
}

// Match with enum and data
let shape = Shape::Rectangle(10.0, 5.0);
match shape {
    Shape::Circle(r) => println!("Circle radius {}", r),
    Shape::Rectangle(l, w) => println!("Rectangle {}x{}", l, w),
    _ => println!("Other shape"),
}
```

**`if let`** — simpler for one pattern:
```rust
if let Some(x) = some_number {
    println!("Got {}", x);
}
```

---

## 10. Package Management (Cargo)

Cargo is Rust's build system and package manager.

- Create project: `cargo new project_name`
- Build: `cargo build` (dev) or `cargo build --release` (optimized)
- Run: `cargo run`
- Test: `cargo test`
- Check: `cargo check` (fast compile check without binary)

**Dependencies** — add to `Cargo.toml`:
```toml
[dependencies]
rand = "0.8"
```

Then in code:
```rust
use rand::Rng;

let num = rand::thread_rng().gen_range(1..=100);
println!("Random: {}", num);
```

**Cargo.toml** explains: `[package]` metadata, `[dependencies]` external crates.