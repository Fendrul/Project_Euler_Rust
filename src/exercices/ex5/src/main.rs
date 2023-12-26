use utils::smallest_multiple::find_smallest_multiple;

static PARAMETER: u64 = 20;

fn main() {
    let result = find_smallest_multiple(PARAMETER);

    println!("\nSmallest number without remainder: {}", result);
}
