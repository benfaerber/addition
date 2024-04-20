#![allow(dead_code)]
/// ! Boolean logic implemented in Rust to show the power of pattern matching

#[derive(Debug, PartialEq)]
pub enum Boolean {
    True,
    False,
}
use Boolean::*;

impl Boolean {
    pub fn and(&self, other: Self) -> Self {
        match (self, other) {
            (True, True) => True,
            _ => False,
        }
    }

    pub fn or(&self, other: Self) -> Self {
        match (self, other) {
            (_, True) | (True, _) => True,
            _ => False,
        }
    }

    pub fn not(&self) -> Self {
        match self {
            True => False,
            False => True,
        }
    }

    pub fn eq(&self, other: Self) -> Self {
        match (self, other) {
            (True, True) | (False, False) => True,
            _ => False,
        }
    }

    pub fn neq(&self, other: Self) -> Self {
        self.eq(other).not()
    }
}


#[cfg(test)]
mod test {
    use super::Boolean::*;

    #[test]
    fn boolean_logic_test() {
        // And
        assert_eq!(True.and(False), False);
        assert_eq!(True.and(True), True);
        assert_eq!(False.and(False), False);
        
        // Or
        assert_eq!(True.or(False), True);
        assert_eq!(False.or(True), True);
        assert_eq!(False.or(False), False);
       
        // Not
        assert_eq!(False.not(), True);
        assert_eq!(True.not(), False);
       
        // Equal
        assert_eq!(False.eq(False), True);
        assert_eq!(True.eq(True), True);
        assert_eq!(False.eq(True), False);
        assert_eq!(True.eq(False), False);
        
        // Neq
        assert_eq!(False.neq(True), True);
        assert_eq!(True.neq(True), False);
        assert_eq!(True.neq(False), True);
    }
}
