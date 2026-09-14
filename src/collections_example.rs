use std::collections::HashMap;

pub fn run() {
    println!("\n=== Collections Example ===\n");

    // Vec<T> = dynamic array
    let mut numbers = vec![10, 20, 30];
    numbers.push(40);
    numbers.push(50);

    println!("Numbers: {:?}", numbers);
    println!("First number: {}", numbers[0]);
    println!("Length: {}", numbers.len());

    // Iterate over a vector
    for number in &numbers {
        println!("Value: {}", number);
    }

    // HashMap<K, V>
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 95);

    println!("Scores: {:?}", scores);
    println!("Alice score: {}", scores.get("Alice").copied().unwrap_or(0));

    // Iterate over HashMap entries
    for (name, score) in &scores {
        println!("{} scored {}", name, score);
    }

    // String operations
    let mut message = String::from("hello");
    message.push(' ');
    message.push_str("world");
    println!("Message: {}", message);
    println!("Contains 'world': {}", message.contains("world"));
}
