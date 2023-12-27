use prime_tools::is_u32_prime;
use utils::permuter::permute;

fn main() {
    let mut max_prime = 0;


    for array_length in 2..9 {
        let mut digits: Vec<u32> = initalize_arrray(array_length);

        let permutations = permute(&mut digits);

        permutations.iter()
            .map(|digits| {
                let mut number = 0;
                for digit in digits {
                    number = number * 10 + digit;
                }
                number
            })
            .filter(|&number| is_u32_prime(number))
            .for_each(|number| {
                if number > max_prime {
                    max_prime = number;
                }
            });
    }

    println!("The largest pandigital prime is {}", max_prime);
}

fn initalize_arrray(array_length: i32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    for i in 1..=array_length {
        digits.push(i as u32);
    }
    digits
}