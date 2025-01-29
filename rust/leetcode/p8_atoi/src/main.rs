pub fn my_atoi(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let s = s.as_bytes();
    let mut i = 0;

    // Whitespaces
    while i < s.len() {
        if s[i] != b' ' {
            break;
        }
        i += 1;
    }

    if i >= s.len() {
        return 0;
    }

    // Signedness
    let positive = if s[i] == b'-' {
        i += 1;
        false
    } else if s[i] == b'+' {
        i += 1;
        true
    } else {
        true
    };

    // Leading zeros
    let mut result: i32 = loop {
        if i >= s.len() {
            return 0;
        }
        if s[i] == b'0' {
            i += 1;
            continue;
        }
        if !s[i].is_ascii_digit() {
            return 0;
        } else {
            break (s[i] - b'0') as i32;
        }
    };
    i += 1;

    if !positive {
        result *= -1;
    }

    // Conversion
    while i < s.len() {
        dbg!(result);
        if !s[i].is_ascii_digit() {
            break;
        }
        result = result.saturating_mul(10);
        let n = (s[i] - b'0') as i32;
        result = if positive {
            result.saturating_add(n)
        } else {
            result.saturating_sub(n)
        };
        i += 1;
    }
    result
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l1() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }
    #[test]
    fn l2() {
        assert_eq!(my_atoi("-042".to_string()), -42);
    }
    #[test]
    fn l3() {
        assert_eq!(my_atoi("1337c0d3".to_string()), 1337);
    }
    #[test]
    fn l4() {
        assert_eq!(my_atoi("0-1".to_string()), 0);
    }
    #[test]
    fn l5() {
        assert_eq!(my_atoi("words and 987".to_string()), 0);
    }

    #[test]
    fn lb1() {
        assert_eq!(my_atoi("".to_string()), 0);
        assert_eq!(my_atoi(" ".to_string()), 0);
        assert_eq!(my_atoi("+".to_string()), 0);
        assert_eq!(my_atoi("-".to_string()), 0);
    }
}
