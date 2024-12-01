use std::env;
use std::fs::OpenOptions;
use std::io::Read;

pub fn get_input(path: &str) -> String {
    let mut path_buf = env::current_dir().expect("Unable to get current working directory!");
    path_buf.push("inputs");
    path_buf.push(path);

    let mut file = OpenOptions::new()
        .read(true)
        .open(path_buf)
        .expect("Unable to open file!");

    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Unable to read from file!");
    string
}
