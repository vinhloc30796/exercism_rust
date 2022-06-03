// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    //! Divide `dividend` by `divisor`, returning the quotient and remainder.
    let quotient: i16 = dividend / divisor;
    let remainder: i16 = dividend % divisor;
    (quotient, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    //! Return an iterator that yields the even-positioned values of `iter`.
    iter.enumerate().filter_map(move |(i, x)| if i % 2 == 0 { Some(x) } else { None })
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        //! Return the Manhattan distance between `self` and (0, 0).
        (self.0.abs() + self.1.abs())
    }
}
