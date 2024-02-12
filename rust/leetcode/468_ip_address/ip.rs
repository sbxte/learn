
fn validate_v4(query_ip: String) -> String {
    let mut count = 0;
    for s in query_ip.split('.') {
        let len = s.len();
        if len == 0 || len > 3 {
            return "Neither".to_string()
        }

        // No leading zeros
        if len != 1 && s.chars().nth(0).unwrap() == '0' {
            return "Neither".to_string()
        }

        // Account for alphabetical letters
        let n = match s.parse::<u16>() {
            Err(_) => return "Neither".to_string(),
            Ok(n) => n,
        };

        if n > 255 {
            return "Neither".to_string();
        }

        count += 1;
    }

    if count != 4 {
        return "Neither".to_string();
    }

    "IPv4".to_string()
}

fn validate_v6(query_ip: String) -> String {
    let mut count = 0;
    for s in query_ip.split(':') {
    let len = s.len();
        if len == 0 || len > 4 {
            return "Neither".to_string()
        }

        if let Err(_) = u16::from_str_radix(s, 16) {
            return "Neither".to_string();
        }

        count += 1;
    }

    if count != 8 {
        return "Neither".to_string();
    }

    "IPv6".to_string()
}

struct Solution {}

impl Solution {
    fn valid_ip_address(query_ip: String) -> String {
        let len = query_ip.len();
        if len < 7 || len > 40 {
            return "Neither".to_string()
        }

        enum Guess {
            V4, V6, Neither
        }

        let mut guess: Guess = Guess::Neither;

        // Parse out first 5 characters
        // If the string is indeed a VALID ip address then v4 and v6 characteristics
        // such as dots and colons MUST show up here
        for c in query_ip[0..5].chars() {
            if !matches!(c.to_ascii_lowercase(), 'a'..='f' | ':' | '.' | '0'..='9') { return "Neither".to_string() }

            if c == '.' {
                guess = Guess::V4;
            } else if matches!(c, 'a'..='f' | ':') {
                guess = Guess::V6;
            }
        }

        return match guess {
            Guess::Neither => "Neither".to_string(),
            Guess::V4 => validate_v4(query_ip),
            Guess::V6 => validate_v6(query_ip)
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn gibberish() {
        assert_eq!(Solution::valid_ip_address("".to_string()), "Neither");
        assert_eq!(Solution::valid_ip_address("0123456789".to_string()), "Neither");
        assert_eq!(Solution::valid_ip_address("asjdfkalsjdfalsdkfjalskdfj".to_string()), "Neither");
    }

    #[test]
    fn v4() {
        assert_eq!(Solution::valid_ip_address("1.1.1.1".to_string()), "IPv4");
        assert_eq!(Solution::valid_ip_address("172.16.254.1".to_string()), "IPv4");
        assert_eq!(Solution::valid_ip_address("172.016.254.1".to_string()), "Neither");
        assert_eq!(Solution::valid_ip_address("172.16.254.001".to_string()), "Neither");

        assert_eq!(Solution::valid_ip_address("255.255.255.255".to_string()), "IPv4");
        assert_eq!(Solution::valid_ip_address("256.256.256.256".to_string()), "Neither");

        assert_eq!(Solution::valid_ip_address("12a.23e.aa6.1d0".to_string()), "Neither");

        assert_eq!(Solution::valid_ip_address("12.12.12.12.12".to_string()), "Neither");
    }

    #[test]
    fn v6() {
        assert_eq!(Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string()), "IPv6");
        assert_eq!(Solution::valid_ip_address("20EE:Fb8:85a3:0:0:8A2E:0370:7334".to_string()), "IPv6");
        assert_eq!(Solution::valid_ip_address("2001.0db8:85a3:0:0:8A2E:0370:7334".to_string()), "Neither");
        assert_eq!(Solution::valid_ip_address("200x:0db8:85a3:0:0:8A2E:0370:7334".to_string()), "Neither");
    }
}
