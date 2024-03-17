use std::mem::size_of;

fn main() {
    // Basic data types
    println!("Basic data types");
    println!("i32: {}", size_of::<i32>());
    println!("u32: {}", size_of::<u32>());
    println!("usize: {}", size_of::<usize>());

    // Pointer sizes
    // Interestingly, these should all be the same. Demonstrating rust's zero-cost policy.
    println!("\ni32 Pointers");
    println!("&i32: {}", size_of::<&i32>());
    println!("&mut i32: {}", size_of::<&mut i32>());
    println!("*const i32: {}", size_of::<*const i32>());
    println!("*mut i32: {}", size_of::<*mut i32>());
}
