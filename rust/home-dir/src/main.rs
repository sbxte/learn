use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use anyhow::Result;

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let mut path = if let Some(x) = home::home_dir() {
        x
    } else {
        panic!("Home dir unavailable");
    };

    path.push(".helloworld");

    println!("Path is {:?}", path.to_str());
    let mut file = if !path.exists() {
        println!("File did not exists, creating new file!");
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)?;
        f.write_all(b"Hello, World!")?;
        f
    } else {
        OpenOptions::new().append(true).read(true).open(path)?
    };
    let mut content = String::new();
    let _ = file.read_to_string(&mut content)?;
    println!("Contents of the file: {content}");
    Ok(())
}
