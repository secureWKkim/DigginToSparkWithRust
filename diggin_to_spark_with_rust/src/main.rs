mod context;

use context::{SparkContext};

fn main() {
    let sc = SparkContext {
        master: String::from("localhost:8080"),
        py_files: Vec::new(),
    };
    println!("Hello, world!");
}
