use tokio::time::{sleep, Duration};

pub async fn run() {
    println!("\n=== Real Async Example with Tokio ===\n");

    let task_one = async {
        println!("Task 1 started");
        sleep(Duration::from_millis(200)).await;
        println!("Task 1 finished");
        "one"
    };

    let task_two = async {
        println!("Task 2 started");
        sleep(Duration::from_millis(100)).await;
        println!("Task 2 finished");
        "two"
    };

    let (result1, result2) = tokio::join!(task_one, task_two);
    println!("Results: {} and {}", result1, result2);

    let fetch_data = async {
        sleep(Duration::from_millis(300)).await;
        String::from("user data")
    };

    let processed = async {
        let data = fetch_data.await;
        format!("processed: {}", data)
    };

    println!("Final async result: {}", processed.await);
}
