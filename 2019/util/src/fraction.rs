use std::cmp::{max, min};

#[derive(PartialEq, Debug)]
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Fraction {
        Fraction {
            numerator,
            denominator,
        }
    }

    pub fn reduce(&self) -> Fraction {
        if self.numerator == 0 || self.denominator == 0 {
            return Fraction::new(self.numerator, self.denominator);
        }
        let gcd = gcd(self.numerator, self.denominator);
        Fraction {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

pub fn gcd(a: i32, b: i32) -> i32 {
    let mut r = a.abs();
    let mut s = b.abs();
    loop {
        let c_min = min(r, s);
        let c_max = max(r, s);
        let d = c_max % c_min;
        if d == 0 {
            break c_min;
        }
        r = c_min;
        s = d;
    }
}

#[test]
fn test_gcd() {
    assert_eq!(6, gcd(48, 18));
    assert_eq!(1, gcd(7, 18));
    assert_eq!(18, gcd(72, 54));
    assert_eq!(18, gcd(72, 54));
}

#[test]
fn fraction_reduce() {
    assert_eq!(Fraction::new(4, 7), Fraction::new(40, 70).reduce());
    assert_eq!(Fraction::new(3, 5), Fraction::new(42, 70).reduce())
}

#[test]
fn fraction_reduce_negativ() {
    assert_eq!(Fraction::new(-2, 1), Fraction::new(-4, 2).reduce());
}
