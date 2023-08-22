use std::fs::File;

//TODO: String 스택/힙 할당 & 5.1 맨 밑 구조체의 소유권 내용 다루기.

pub struct SparkContext {
    pub master: String,
    pub py_files: Vec<File>
}