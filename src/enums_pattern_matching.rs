// Enums are a core Rust feature for modeling data with different variants.
enum HttpStatus {
    Ok,
    NotFound,
    ServerError,
}

enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(base, height) => 0.5 * base * height,
        }
    }
}

fn describe_status(status: HttpStatus) -> &'static str {
    match status {
        HttpStatus::Ok => "Request succeeded",
        HttpStatus::NotFound => "Resource missing",
        HttpStatus::ServerError => "Server crashed",
    }
}

pub fn run() {
    println!("\n=== Enums and Pattern Matching ===\n");

    let status = HttpStatus::Ok;
    println!("Status: {}", describe_status(status));

    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle { width: 4.0, height: 6.0 };
    let triangle = Shape::Triangle(3.0, 8.0);

    println!("Circle area: {:.2}", circle.area());
    println!("Rectangle area: {:.2}", rectangle.area());
    println!("Triangle area: {:.2}", triangle.area());

    // Pattern matching using if let
    if let Shape::Circle(radius) = circle {
        println!("Circle has radius {:.2}", radius);
    }
}
