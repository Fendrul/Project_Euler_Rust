fn main() {
    let mut max = 0;

    for x in 100..999 {
        for y in x..999 {

            let product = x * y;

            if is_palindrome(product) {
                if product > max {
                    max = product;
                }
            }
        }
    }

    println!("\nMax palindrome: {}", max);
}

fn is_palindrome(number: i32) -> bool {
    let mut reversed = 0;
    let mut n = number;

    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }

    return number == reversed;
}
