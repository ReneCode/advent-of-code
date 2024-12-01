// math

// least common multiplier
pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

// greatest common divider:
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod test {
    use crate::util::math::lcm;

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(4, 5), 20);
        assert_eq!(lcm(4, 6), 12);
    }
}
