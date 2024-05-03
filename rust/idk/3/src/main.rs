#![allow(invalid_reference_casting)]
fn main() {
    let x = 5;
    let y = &x;

    // This is considered undefined behaviour
    let z = y as *const i32 as *mut i32;
    unsafe {
        *z = 10;
    }
    println!("x is: {}", x);
}
