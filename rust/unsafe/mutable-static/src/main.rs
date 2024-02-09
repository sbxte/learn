static mut NUMBER: i32 = 5;

fn main() {
	unsafe { // Mutable static variable
		println!("Mutable is {}", NUMBER);
		NUMBER = 10;
		println!("Mutable is {}", NUMBER);
	}
}
