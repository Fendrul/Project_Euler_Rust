use utils::prime_number_utils::PrimeNumberUtil;

static PARAMETER: i32 = 1000001;

fn main() {
    let prime_number_util = PrimeNumberUtil::new();

    let result = prime_number_util
        .take(PARAMETER as usize)
        .last()
        .unwrap();

    println!("{}", result);
}
