// Traits define shared behavior for different types.
trait Animal {
    fn sound(&self) -> &str;
    fn speak(&self) {
        println!("{}", self.sound());
    }
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn sound(&self) -> &str {
        "Woof!"
    }
}

impl Animal for Cat {
    fn sound(&self) -> &str {
        "Meow!"
    }
}

// Generic function using a trait bound.
fn describe<T: Animal>(animal: &T) {
    println!("The animal says: {}", animal.sound());
}

pub fn run() {
    println!("\n=== Trait Example ===\n");

    let dog = Dog;
    let cat = Cat;

    dog.speak();
    cat.speak();

    describe(&dog);
    describe(&cat);
}
