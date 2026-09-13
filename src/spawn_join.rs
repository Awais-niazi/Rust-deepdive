use std::thread;
use std::time::Duration;

pub fn run() {
    println!("\n=== Spawn and Join in Rust ===\n");

    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Child thread: step {}", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Main thread keeps running
    for i in 1..=3 {
        println!("Main thread: step {}", i);
        thread::sleep(Duration::from_millis(300));
    }

    // Join waits for the spawned thread to finish before continuing.
    handle.join().unwrap();
    println!("Both threads finished.");

    // Example with data passed into the thread
    let data = vec![1, 2, 3, 4, 5];
    let handle2 = thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        println!("Sum from spawned thread: {}", sum);
    });

    handle2.join().unwrap();

    // Example: waiting for result from thread
    let handle3 = thread::spawn(|| {
        let result = 42;
        result
    });

    let value = handle3.join().unwrap();
    println!("Result from thread: {}", value);
}
