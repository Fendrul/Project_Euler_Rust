static MAX_NUMBER: u32 = 123_456_789;

fn main() {
    let mut max_number = 0;
    let max_length = MAX_NUMBER.to_string().len() / 2;
    println!("max_length: {}", max_length);

    for number in 2..10_i32.pow(max_length as u32) {
        let mut digits: Vec<u32> = Vec::new();
        let mut products: Vec<i32> = Vec::new();

        let mut factor = 1;
        while digits.len() < 9 {
            let product_to_add = number * factor;
            products.push(product_to_add);

            digits
                .extend(
                    product_to_add.to_string()
                        .chars()
                        .map(|c| c.to_digit(10)
                            .unwrap())
                );

            factor += 1;
        }

        if digits.len() == 9 && digits.iter().all(|&d| d != 0) {
            let mut sorted_digits = digits.to_vec();
            let pandigital_number = sorted_digits.iter().fold(0, |acc, &d| acc * 10 + d);
            sorted_digits.sort();
            sorted_digits.dedup();

            if sorted_digits.len() == 9  && pandigital_number > max_number {
                max_number = pandigital_number;
                println!("{} makes a pandigital with the products {:?}", pandigital_number, products);
            }
        }
    }
}
