// Option and Result are two of the most important Rust enums.
// Option<T> = Some(T) or None
// Result<T, E> = Ok(T) or Err(E)

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero is not allowed."))
    } else {
        Ok(a / b)
    }
}

fn find_user(name: &str) -> Option<&str> {
    match name {
        "admin" => Some("admin"),
        "guest" => Some("guest"),
        _ => None,
    }
}

pub fn run() {
    println!("\n=== Option and Result Example ===\n");

    // Option example
    let admin = find_user("admin");
    let unknown = find_user("unknown");

    println!("admin: {:?}", admin);
    println!("unknown: {:?}", unknown);

    // Matching on Option
    match admin {
        Some(user) => println!("Logged in as: {}", user),
        None => println!("No user found."),
    }

    // Result example
    let good = divide(10, 2);
    let bad = divide(10, 0);

    println!("good: {:?}", good);
    println!("bad: {:?}", bad);

    match good {
        Ok(value) => println!("Division result: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    match bad {
        Ok(value) => println!("Division result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
