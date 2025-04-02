mod tasks;
mod storage;
mod utils;

use tasks::Task;
use utils::print_welcome;

fn main() {
    print_welcome();
    let task = Task::new(1, "Learn Rust".to_string());
    println!("{:?}", task);
}


