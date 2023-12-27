use getset::{Getters, Setters};

#[derive(Debug, Getters, Setters)]
pub struct Triangle {
    #[getset(get = "pub")]
    a: u32,
    #[getset(get = "pub")]
    b: u32,
    #[getset(get = "pub")]
    c: u32,
}

impl Triangle {
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        Self { a, b, c }
    }

    pub fn is_pythagorean(&self) -> bool {
        self.a.pow(2) + self.b.pow(2) == self.c.pow(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pythagorean() {
        run_test_is_pythagorean(20, 48, 52, true);
        run_test_is_pythagorean(24, 45, 51, true);
        run_test_is_pythagorean(30, 40, 50, true);
        run_test_is_pythagorean(20, 48, 53, false);
    }

    fn run_test_is_pythagorean(a: u32, b: u32, c: u32, expected: bool) {
        let triangle = Triangle::new(a, b, c);
        assert_eq!(triangle.is_pythagorean(), expected);
    }
}