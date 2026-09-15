pub fn run() {
    println!("\n=== Ownership and Borrowing in Rust ===\n");

    // Ownership: a value can have only one owner at a time.
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 after move: {}", s2);
    // println!("s1: {}", s1); // compile error: s1 was moved into s2

    // Function ownership transfer
    let s3 = String::from("rust");
    takes_ownership(s3);
    // println!("s3: {}", s3); // compile error: moved into function

    // Borrowing: references allow access without transferring ownership
    let mut message = String::from("hello");

    let r1 = &message;
    let r2 = &message;
    println!("Immutable borrows: {} and {}", r1, r2);

    // Mutable borrow: only one mutable reference allowed at a time
    let r3 = &mut message;
    r3.push('!');
    println!("After mutable borrow: {}", r3);

    // A larger example: building a text summary using borrowed slices.
    let blog_text = String::from("Rust ownership and borrowing are powerful ideas");
    let title = first_word(&blog_text);
    let preview = preview_text(&blog_text);
    println!("Title: {}", title);
    println!("Preview: {}", preview);

    // Borrowing from a vector by slice
    let numbers = vec![10, 20, 30, 40];
    println!("Sum of numbers: {}", sum(&numbers));

    // Another realistic example: filtering by a borrowed slice.
    let names = vec!["Awais", "Sam", "Zain"];
    let chosen = pick_name(&names, 1);
    println!("Chosen name: {}", chosen);

    // Common compile-time ownership/borrowing mistakes:
    // Uncomment these blocks one at a time to see the compiler error they trigger.
    // 1) Using a value after it has been moved.
    // let moved_value = String::from("bad");
    // let another_owner = moved_value;
    // println!("{}", moved_value);

    // 2) Mutating a value while an immutable reference is still active.
    // let mut text = String::from("hello");
    // let r = &text;
    // text.push('!');
    // println!("{}", r);

    // 3) Having two mutable references to the same value at once.
    // let mut numbers = vec![1, 2, 3];
    // let a = &mut numbers;
    // let b = &mut numbers;
    // a.push(4);
    // b.push(5);
}

fn takes_ownership(value: String) {
    println!("Inside function, owned value: {}", value);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}

fn preview_text(s: &str) -> &str {
    let trimmed = s.trim();
    let bytes = trimmed.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b'.' {
            return &trimmed[..index + 1];
        }
    }

    trimmed
}

fn pick_name<'a>(names: &'a [&'a str], index: usize) -> &'a str {
    names[index]
}

fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}
