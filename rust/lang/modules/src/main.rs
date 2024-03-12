mod foo;
use foo::bar::*;
use foo::*;

fn main() {
    println!("Hello, world!");
    foo();
    bar();
}
