static PARAMETER: u64 = 100;

fn main() {
    let sum: u64 = PARAMETER * (PARAMETER + 1) / 2;

    let square_of_sum: u64 = sum * sum;
    let sum_of_square: u64 = (PARAMETER * (PARAMETER + 1) * (2 * PARAMETER + 1)) / 6; // le calcul se fait grâce à la formule de Faulhaber

    println!("the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum is {}", square_of_sum - sum_of_square);
}
