// ============================================================================
// RUST CLASS — All Concepts Demonstrated
// ============================================================================

use rand::Rng;

// ========================
// STRUCTS
// ========================

// Define a struct to group related data
struct User {
    name: String,
    age: u32,
    designation: String,
}

// Tuple struct (fields accessed by index)
struct Color(u8, u8, u8);

// ========================
// ENUMS
// ========================

// Enum — a variable can be one of several possible values
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Enum with data attached to each variant
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}


// ============================================================================
// FUNCTION DEFINITIONS
// ============================================================================

// A simple function that takes two params and returns their sum
fn add(a: i32, b: i32) -> i32 {
    a + b  // implicit return (no semicolon)
}

// Function with no return value (returns unit type ())
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function that returns a boolean — expression style
fn is_even(num: i32) -> bool {
    num % 2 == 0  // this expression is the return value
}

// Fibonacci function using a loop
fn fib(n: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if n == 0 || n == 1 {
        return n;
    }

    for _ in 0..(n - 2) {
        let temp = second;
        second = first + second;
        first = temp;
    }

    second
}

// Function taking a string slice and returning its length
fn get_str_len(s: &str) -> usize {
    s.chars().count()
}

// Function demonstrating Option<T> — returns Some or None
fn find_item(items: &[&str], target: &str) -> Option<usize> {
    for (index, &item) in items.iter().enumerate() {
        if item == target {
            return Some(index); // found it
        }
    }
    None // not found
}

// Function demonstrating Result<T, E> — returns Ok or Err
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// ============================================================================
// CONCEPT FUNCTIONS — each one demonstrates a Rust concept
// ============================================================================

// ---------------------------------------------------------------------------
// 1. VARIABLES — immutable by default, mutable with `mut`, shadowing, constants
// ---------------------------------------------------------------------------
fn demonstrate_variables() {
    println!("=== 1. Variables ===");

    // Immutable variable (cannot change)
    let x = 5;
    println!("x = {}", x);

    // Mutable variable (CAN change)
    let mut y = 10;
    println!("y before = {}", y);
    y = 15;
    println!("y after = {}", y);

    // Constant — always immutable, must have type annotation
    const MAX_POINTS: u32 = 100_000;
    println!("Max points constant = {}", MAX_POINTS);

    // Variable shadowing — reuse the same name
    let z = 5;
    println!("z originally = {}", z);
    let z = z + 1;
    println!("z shadowed = {}", z);

    // Different data types
    let active: bool = true;
    let temperature: f64 = 36.6;
    let letter: char = 'A';
    let name: &str = "Sudhir";
    println!(
        "active={}, temp={}, letter={}, name={}",
        active, temperature, letter, name
    );
}

// ---------------------------------------------------------------------------
// 2. FUNCTIONS — params, return values, expression bodies
// ---------------------------------------------------------------------------
fn demonstrate_functions() {
    println!("=== 2. Functions ===");

    let sum = add(10, 20);
    println!("add(10, 20) = {}", sum);

    greet("Sudhir");

    let even = is_even(401);
    println!("is_even(401) = {}", even);

    let fib_num = fib(7);
    println!("fib(7) = {}", fib_num);

    let len = get_str_len("Hello, Rust!");
    println!("Length = {}", len);
}

// ---------------------------------------------------------------------------
// 3. CONDITIONALS — if / else if / else, if as an expression
// ---------------------------------------------------------------------------
fn demonstrate_conditionals() {
    println!("=== 3. Conditionals ===");

    let age = 25;

    if age >= 18 {
        println!("You are an adult");
    } else if age >= 13 {
        println!("You are a teenager");
    } else {
        println!("You are a child");
    }

    // `if` as an expression — both branches must return the same type
    let status = if age >= 18 { "adult" } else { "minor" };
    println!("Status: {}", status);
}

// ---------------------------------------------------------------------------
// 4. LOOPS — loop (infinite), while, for
// ---------------------------------------------------------------------------
fn demonstrate_loops() {
    println!("=== 4. Loops ===");

    // `loop` — runs forever until `break`
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            break;
        }
    }
    println!("Loop counter = {}", counter);

    // `while` — keeps running while condition is true
    let mut num = 3;
    while num != 0 {
        println!("{}!", num);
        num -= 1;
    }

    // `for` — iterate over a range (0 to 3)
    for i in 0..4 {
        println!("for i = {}", i);
    }

    // Iterate over items in a slice
    for item in ["apple", "banana", "cherry"].iter() {
        println!("fruit = {}", item);
    }
}

// ---------------------------------------------------------------------------
// 5. STRUCTS — create and use
// ---------------------------------------------------------------------------
fn demonstrate_structs() {
    println!("=== 5. Structs ===");

    let user = User {
        name: String::from("sudhir"),
        age: 25,
        designation: String::from("Software Engineer"),
    };

    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("Designation: {}", user.designation);

    // Tuple struct — access by index
    let dark_gray = Color(50, 50, 50);
    println!("Color RGB = ({}, {}, {})", dark_gray.0, dark_gray.1, dark_gray.2);
}

// ---------------------------------------------------------------------------
// 6. ENUMS — define with variant data, match them
// ---------------------------------------------------------------------------
fn demonstrate_enums() {
    println!("=== 6. Enums ===");

    let direction = Direction::Left;
    match direction {
        Direction::Up => println!("Going up"),
        Direction::Down => println!("Going down"),
        Direction::Left => println!("Going left"),
        Direction::Right => println!("Going right"),
    }

    let shape = Shape::Circle(3.5);
    let area = match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
    };
    println!("Shape area = {:.2}", area);
}

// ---------------------------------------------------------------------------
// 7. OPTION<T> — Some or None
// ---------------------------------------------------------------------------
fn demonstrate_option() {
    println!("=== 7. Option<T> ===");

    let fruits = ["apple", "banana", "cherry"];

    // find_item returns Option<usize>
    match find_item(&fruits, "banana") {
        Some(index) => println!("Found 'banana' at index {}", index),
        None => println!("'banana' not found"),
    }

    match find_item(&fruits, "mango") {
        Some(index) => println!("Found 'mango' at index {}", index),
        None => println!("'mango' not found"),
    }

    // unwrap_or — provide a default if None
    let result = find_item(&fruits, "mango").unwrap_or(0);
    println!("'mango' index (unwrapped) = {}", result);
}

// ---------------------------------------------------------------------------
// 8. RESULT<T, E> — Ok or Err
// ---------------------------------------------------------------------------
fn demonstrate_result() {
    println!("=== 8. Result<T, E> ===");

    match divide(10.0, 2.0) {
        Ok(result) => println!("10/2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10/0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

// ---------------------------------------------------------------------------
// 9. PATTERN MATCHING — match statements
// ---------------------------------------------------------------------------
fn demonstrate_pattern_matching() {
    println!("=== 9. Pattern Matching ===");

    let number = 13;

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("Prime"),
        13..=19 => println!("A teen"),
        _ => println!("Something else"),
    }

    // `if let` — shortcut for matching one pattern
    let some_val = Some(42);
    if let Some(v) = some_val {
        println!("Value inside Some = {}", v);
    }
}

// ---------------------------------------------------------------------------
// 10. PACKAGE MANAGEMENT — using external crate (rand)
// ---------------------------------------------------------------------------
fn demonstrate_package_management() {
    println!("=== 10. Package Management (rand crate) ===");

    // Use rand crate functions (imported at top)
    let random_num = rand::thread_rng().gen_range(1..=100);
    println!("Random number between 1-100 = {}", random_num);
}

// ============================================================================
// MAIN — calls all concept functions
// ============================================================================
fn main() {
    println!("Hello, world!\n");

    // 1. Variables
    demonstrate_variables();

    // 2. Functions
    demonstrate_functions();

    // 3. Conditionals
    demonstrate_conditionals();

    // 4. Loops
    demonstrate_loops();

    // 5. Structs
    demonstrate_structs();

    // 6. Enums
    demonstrate_enums();

    // 7. Option<T>
    demonstrate_option();

    // 8. Result<T, E>
    demonstrate_result();

    // 9. Pattern Matching
    demonstrate_pattern_matching();

    // 10. Package Management
    demonstrate_package_management();

    // Original struct usage (your previous code)
    let user = User {
        name: String::from("sudhir"),
        age: 25,
        designation: String::from("Software Engineer"),
    };
    println!("\nUser name is {}", user.name);
}