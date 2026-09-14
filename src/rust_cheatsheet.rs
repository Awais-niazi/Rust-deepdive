pub fn run() {
    println!("\n=== Rust Cheat Sheet ===\n");

    println!("1. Variables");
    let immutable = 10;
    let mut mutable = 20;
    mutable += 5;
    println!("immutable = {}, mutable = {}", immutable, mutable);

    println!("\n2. Ownership");
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 = {}", s2);

    println!("\n3. Borrowing");
    let message = String::from("rust");
    let len = message.len();
    println!("Message length = {}", len);

    println!("\n4. Functions");
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("add(3, 4) = {}", add(3, 4));

    println!("\n5. Enums and Option/Result");
    let maybe_value: Option<i32> = Some(42);
    let maybe_none: Option<i32> = None;
    println!("maybe_value = {:?}, maybe_none = {:?}", maybe_value, maybe_none);

    let parsed: Result<i32, &str> = Ok(100);
    println!("parsed = {:?}", parsed);

    println!("\n6. Collections");
    let vec = vec![1, 2, 3];
    let mut map = std::collections::HashMap::new();
    map.insert("x", 10);
    println!("vec = {:?}, map = {:?}", vec, map);

    println!("\n7. Traits");
    trait Speak {
        fn speak(&self) -> &str;
    }

    struct Person;
    impl Speak for Person {
        fn speak(&self) -> &str {
            "hello"
        }
    }

    let person = Person;
    println!("person says: {}", person.speak());

    println!("\n8. Async");
    println!("Use async fn and tokio::spawn / tokio::join! for async tasks");

    println!("\n9. Pattern Matching");
    let number = 7;
    let text = match number {
        1 => "one",
        2 => "two",
        _ => "other",
    };
    println!("number = {}, text = {}", number, text);
}
