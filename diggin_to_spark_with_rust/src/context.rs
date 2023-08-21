use std::fs::File;

// #[derive(Debug)]
pub struct SparkContext {
    master: String,
    py_files: Vec<File>
}

impl SparkContext {
    // 인자 아무것도 안 받을 때 어떻게 하면 좋을까.
    // pub fn stop(&self) {}

    pub fn addPyFile(&self, path : &str) {
        //ref: https://doc.rust-lang.org/std/fs/struct.File.html
        // open 메서드 쓰면 .py 파일 경로 입력해야 하는 번거로움이 있어서.
        py_files.push(File::create(path));
    }

    
}