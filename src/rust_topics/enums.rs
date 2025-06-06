// =====================================
//              RUST ENUMS
// =====================================

/*
Enums in Rust are used to define a type which can be one of several variants.
They are powerful and commonly used in pattern matching.
*/

// -------------------------------------
// Example 1: Basic Enum with a Single Variant
// -------------------------------------

#[cfg(feature = "rust_enum")]
pub fn rust_enum() {
    enum Office {
        Employee(String),
    }

    let employee_data = Office::Employee("Alice".to_string());

    match employee_data {
        Office::Employee(employee) => println!("Employee: {}", employee),
    }
}

// -------------------------------------
// Example 2: Enum with Multiple Variants
// -------------------------------------

enum _Direction {
    North,
    South,
    East,
    West,
}

fn _move_direction(dir: _Direction) {
    match dir {
        _Direction::North => println!("Moving north!"),
        _Direction::South => println!("Heading south!"),
        _Direction::East => println!("Going east!"),
        _Direction::West => println!("Turning west!"),
    }
}

// -------------------------------------
// Example 3: Enum with Data in Variants
// -------------------------------------

enum _Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

fn _handle_message(msg: _Message) {
    match msg {
        _Message::Quit => println!("Quit message received."),
        _Message::Write(text) => println!("Message: {}", text),
        _Message::Move { x, y } => println!("Moving to position ({}, {})", x, y),
        _Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}

// -------------------------------------
// Example 4: Enum with Generic Result Type
// -------------------------------------

fn _use_result_enum() {
    let result: Result<i32, &str> = Ok(42);

    match result {
        Ok(val) => println!("Got value: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}
