fn main() {
	// Using unsafe to break borrow checker rules :)

	let mut num = 5;

	let r1 = &num as *const i32;
	let r2 = &mut num as *mut i32;

	unsafe { // Dereferencing raw pointers
		println!("r1 is {}", *r1);
		println!("r2 is {}", *r2);
	}

	unsafe { // Mutating pointer contents
		*r2 = 10;
		println!("r1 is {}", *r1);
		println!("r2 is {}", *r2);
	}
}

