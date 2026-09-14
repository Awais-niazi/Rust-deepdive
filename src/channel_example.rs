use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    println!("\n=== Channel Communication in Rust ===\n");

    // Create a channel: sender and receiver
    let (tx, rx) = mpsc::channel();

    let sender_thread = thread::spawn(move || {
        for i in 1..=5 {
            println!("Sending: {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Receive values from the channel
    for received in rx {
        println!("Received: {}", received);
        if received == 5 {
            break;
        }
    }

    sender_thread.join().unwrap();

    // Another example: sending a String
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let message = String::from("Hello from thread!");
        tx2.send(message).unwrap();
    });

    let received_message = rx2.recv().unwrap();
    println!("Message from thread: {}", received_message);
}
