use primes::{is_prime, PrimeSet, Sieve};

fn main() {
    let mut truncatable_primes: Vec<u64> = Vec::new();
    let mut sieve = Sieve::new();

    let mut primes_iter = sieve.iter();

    while truncatable_primes.len() < 11 {
        let prime = primes_iter.next().unwrap();
        if prime < 8 {
            continue;
        }

        if is_truncatable_number(prime) {
            truncatable_primes.push(prime);
            println!("Found the {} truncatable prime: {}", truncatable_primes.len(), prime);
        }
    }

    let sum: u64 = truncatable_primes.iter().sum();

    println!("Sum of truncatable primes: {}", sum);
}

fn is_truncatable_number(number: u64) -> bool {
    let mut number = number;
    let mut digits = Vec::new();
    while number > 0 {
        digits.push(number % 10);
        number /= 10;
    }
    digits.reverse();

    truncate_number(&digits, true) && truncate_number(&digits, false)
}

fn truncate_number(imported_digits: &[u64], mut truncate_from_left: bool) -> bool {
    let mut digits = imported_digits.to_vec();

    for i in 0..digits.len() - 1 {
        if truncate_from_left {
            digits.remove(0);
        } else {
            digits.pop();
        }

        let number = digits.iter().fold(0, |acc, digit| acc * 10 + digit);

        if !is_prime(number) {
            return false;
        }

        truncate_from_left = !truncate_from_left;
    }

    true
}