use std::thread;
use std::time::Duration;

// This is a simple async-like simulation using threads.
// In Rust, real async/await uses futures and an executor.
pub fn run() {
    println!("\n=== Async/Await-style Flow in Rust ===\n");

    let handle = thread::spawn(|| {
        println!("Task started...");
        thread::sleep(Duration::from_millis(500));
        println!("Task finished after delay");
        String::from("done")
    });

    println!("Main thread continues while task runs...");

    let result = handle.join().unwrap();
    println!("Async task result: {}", result);

    // A small example of async-like chaining using closures.
    let step1 = || {
        println!("Step 1: fetch data");
        String::from("data")
    };

    let step2 = |value: String| {
        println!("Step 2: process {}", value);
        format!("{} processed", value)
    };

    let step3 = |value: String| {
        println!("Step 3: send {}", value);
        value
    };

    let final_value = step3(step2(step1()));
    println!("Pipeline result: {}", final_value);
}
