use std::fmt::{self, Display};

// A trait defines shared behavior.
trait Summary {
    fn summarize(&self) -> String;
}

struct Report {
    title: String,
    score: u32,
}

impl Summary for Report {
    fn summarize(&self) -> String {
        format!("{} (score: {})", self.title, self.score)
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.score)
    }
}

// Lifetime annotation: 'a tells Rust that the returned reference
// must live at least as long as both input references.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Generic function with trait bounds.
fn print_item<T: Display + Summary>(item: &T) {
    println!("Summary: {} | Value: {}", item.summarize(), item);
}

pub fn run() {
    println!("\n=== Advanced Rust Concepts ===\n");

    // Lifetime example
    let first = "short";
    let second = "a much longer string";
    let result = longest(first, second);
    println!("Longest text: {}", result);

    // Trait example
    let report = Report {
        title: String::from("Quarterly Review"),
        score: 95,
    };
    println!("Report summary: {}", report.summarize());

    // Generic + trait bound example
    print_item(&report);

    // Closures can capture surrounding values.
    let multiplier = 3;
    let times_two = |value: i32| value * multiplier;
    println!("Closure result: {}", times_two(10));

    // Example of a higher-order function using closures.
    let numbers = vec![10, 20, 30, 40];
    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    println!("Doubled numbers: {:?}", doubled);
}
