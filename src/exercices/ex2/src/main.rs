use utils::fibonacci::Fibonacci;
fn main() {
    let fib = Fibonacci::new();

    let sum: u32 = fib
        .take_while(|&x| x < 4_000_000)
        .filter(|&x| x % 2 == 0)
        .sum();

    println!("The sum of the even-valued terms of the Fibonacci sequence is {}", sum);
}