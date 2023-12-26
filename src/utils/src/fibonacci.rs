use getset::{Getters, Setters};
//implement the copy trait

#[derive(Debug, Getters, Setters)]
pub struct Fibonacci {
    #[getset(get = "pub")]
    curr: u32,
    #[getset(get = "pub")]
    next: u32,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}