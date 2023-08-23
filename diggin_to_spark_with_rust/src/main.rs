mod context;

use context::{SparkContext};

fn main() {
    let sc = SparkContext { //mut 아니다.
        master: String::from("localhost:8080"),//TODO: String 스택/힙 할당 & 5.1 맨 밑 구조체의 소유권 내용 다루기.
        py_files: Vec::new(),
    };
    println!("Hello, world!");
}
