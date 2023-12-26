use crate::prime_number_utils::PrimeNumberUtil;

pub fn find_smallest_multiple(parametor: u64) -> u64 {
    let mut result: u64 = 1;

    let prime_number_util = PrimeNumberUtil::new();

    let divisors: Vec<u64> = prime_number_util
        .take_while(|&x| x < parametor)
        .collect();

    for prime in divisors {
        let exponant_value  = (parametor as f64).log(prime as f64) as u32;
        result *= prime.pow(exponant_value);
    }

    result
}