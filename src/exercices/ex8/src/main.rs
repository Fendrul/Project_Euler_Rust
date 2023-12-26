use utils::file_reader::FileReader;

static ADJACENT_DIGITS: usize = 13;

fn main() {
    let file_reader = FileReader::new(String::from("src\\exercices\\ex8\\src\\input"));

    let mut max_product: u64 = 0;
    let digits: Vec<u8> = file_reader
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    for digit in 0..digits.len() - ADJACENT_DIGITS {
        if digits[digit] == 0 {
            continue;
        }

        let mut product: u64 = 1;

        for next_digit in digit..digit + ADJACENT_DIGITS {
            if digits[next_digit] == 0 {
                break;
            }

            product *= digits[next_digit] as u64;
        }

        if product > max_product {
            max_product = product;
        }
    }

    println!("Max product: {}", max_product);
}
