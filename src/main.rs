mod advanced_concepts;
mod async_await_example;
mod channel_example;
mod mutex_example;
mod ownership_borrowing;
mod real_async_example;
mod spawn_join;
mod trait_example;

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
    advanced_concepts::run();
    async_await_example::run();
    trait_example::run();

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(real_async_example::run());
}