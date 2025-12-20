use procmacro::HelloMacro;
use procmacro_derive::{HelloMacro, hello_macro_attr};

#[derive(HelloMacro)]
struct Pancakes;

#[hello_macro_attr]
fn halo() {
    println!("Hallo");
}

fn main() {
    Pancakes::hello_macro();
    halo();
}

