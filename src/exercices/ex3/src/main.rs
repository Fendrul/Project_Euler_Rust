use utils::prime_number_utils::PrimeNumberUtil;

static NUMBER_TO_DECOMPOSE: u64 = 600851475143;

fn main() {
    let prime_number = PrimeNumberUtil::new();

    prime_number
        .take_while(|&x| x < (NUMBER_TO_DECOMPOSE as f64).sqrt() as u64)
        .for_each(|x| {
            if NUMBER_TO_DECOMPOSE % x == 0 {
                println!("{}", x);
            }
        });
}
