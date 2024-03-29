pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    println!("Run cargo test!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_add_test() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn bad_add_test() {
        assert_eq!(bad_add(2, 3), 5);
    }
}
