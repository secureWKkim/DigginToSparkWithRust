use std::fs::File;

pub struct SparkContext {
    pub master: String,
    pub py_files: Vec<File>
}