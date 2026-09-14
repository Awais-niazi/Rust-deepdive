mod channel_example;
mod mutex_example;
mod ownership_borrowing;
mod spawn_join;

fn main() {
    // Variables (immutable by default, unlike Python)
    let name = "Awais";
    println!("Hello, {}!", name);

    // Mutable variables (use 'mut' keyword)
    let mut counter = 0;
    counter += 1;
    println!("Counter: {}", counter);

    // Type inference (like Python, but compile-time)
    let x = 5;        // i32 (integer)
    let y = 3.14;     // f64 (float)
    let z = true;     // bool

    println!("x: {}, y: {}, z: {}", x, y, z);

    ownership_borrowing::run();
    spawn_join::run();
    channel_example::run();
    mutex_example::run();
}