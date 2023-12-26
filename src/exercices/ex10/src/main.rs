use primes::{PrimeSet, Sieve};
static MAX: u64 = 2_000_000;

fn main() {
    let mut sieve = Sieve::new();

    let primes = sieve
        .iter()
        .take_while(|&x| x < MAX)
        .sum::<u64>();

    println!("Sum of all primes below 2 million: {}", primes);
}
