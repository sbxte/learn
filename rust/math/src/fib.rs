/// An implementation of the fibonacci sequence generator for only positive n values
pub fn fib_u32(number: u32) -> u32 {
    // fib: 0 1 1 ...
    // n  : 0 1 2 ...
    if number == 0 { return 0; }
    else if number <= 2 { return 1; }

    let mut prev = 1;
    let mut current = 1;
    let mut next;

    for _ in 0..(number - 2) {
        next = prev + current;
        prev = current;
        current = next;
    }

    current
}

/// An implementation of the fibonacci sequence generator for only positive n values
pub fn fib_u64(number: u64) -> u64 {
    // fib: 0 1 1 ...
    // n  : 0 1 2 ...
    if number == 0 { return 0; }
    else if number <= 2 { return 1; }

    let mut prev = 1;
    let mut current = 1;
    let mut next;

    for _ in 0..(number - 2) {
        next = prev + current;
        prev = current;
        current = next;
    }

    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_positive_u32() {
        assert_eq!(0, fib_u32(0));
        assert_eq!(1, fib_u32(1));
        assert_eq!(1, fib_u32(2));
        assert_eq!(2, fib_u32(3));
        assert_eq!(55, fib_u32(10));
        assert_eq!(832040, fib_u32(30));
    }

    #[test]
    fn fib_positive_u64() {
        assert_eq!(0, fib_u64(0));
        assert_eq!(1, fib_u64(1));
        assert_eq!(1, fib_u64(2));
        assert_eq!(2, fib_u64(3));
        assert_eq!(55, fib_u64(10));
        assert_eq!(832040, fib_u64(30));
        assert_eq!(12586269025, fib_u64(50));
    }
}
