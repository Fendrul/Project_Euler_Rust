pub struct PrimeNumberUtil {
    prime_number: Vec<u64>,
}

impl PrimeNumberUtil {
    pub fn new() -> Self {
        PrimeNumberUtil {
            prime_number: Vec::new(),
        }
    }
}

impl Default for PrimeNumberUtil {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for PrimeNumberUtil {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prime_number.is_empty() {
            self.prime_number.push(2);
            return Some(2);
        }

        let mut number: u64 = self.prime_number.last().unwrap() + 1;

        while self.prime_number.iter().any(|&x| number % x == 0 && x <= ((number as f64).sqrt() as u64)) {
            number += 1;
        }

        self.prime_number.push(number);

        Some(number)
    }
}