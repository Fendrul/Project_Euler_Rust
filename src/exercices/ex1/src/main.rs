fn main() {
    let mut multiples = Vec::new();

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            multiples.push(i);
        }
    }

    let sum: u32 = multiples.iter().sum();

    println!("The sum of all the multiples of 3 or 5 below 1000 is {}", sum);
}
