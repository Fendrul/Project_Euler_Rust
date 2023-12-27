static MAX_NUMBER: i32 = 100_000;

fn main() {
    let mut champernowne_constant: Vec<char> = Vec::with_capacity(MAX_NUMBER as usize);

    let mut number = 1;
    while champernowne_constant.len() < MAX_NUMBER as usize {
        champernowne_constant.extend(number.to_string().chars());
        number += 1;
    }

    println!("The product of the digits at the following positions: 1, 10, 100, 1000, 10000, 100000 is {}",
        champernowne_constant[0].to_digit(10).unwrap() *
        champernowne_constant[9].to_digit(10).unwrap() *
        champernowne_constant[99].to_digit(10).unwrap() *
        champernowne_constant[999].to_digit(10).unwrap() *
        champernowne_constant[9999].to_digit(10).unwrap() *
        champernowne_constant[99999].to_digit(10).unwrap()
    );
}
