mod triangle;

static MAX_PERIMETER: u32 = 1000;

fn main() {
    let mut sums: Vec<(u32, u32)> = Vec::new();
    for perimeter in 3..MAX_PERIMETER + 1 {
        let sum = get_number_of_right_angled_triangles(perimeter);

        sums.push((sum, perimeter));
    }

    let (_max, perimeter) = sums.iter().max().unwrap();

    println!("The perimeter with the most right-angled triangles is: {}", perimeter);
}

fn get_number_of_right_angled_triangles(perimeter: u32) -> u32 {
    let mut counter = 0;

    for c in perimeter / 3..perimeter {
        for b in 1..((perimeter - c) / 2) + 1 {
            let a = perimeter - b - c;
            // println!("a: {}, b: {}, c: {}", a, b, c);
            let triangle = triangle::Triangle::new(a, b, c);
            if triangle.is_pythagorean() {
                counter += 1;
            }
        }
    }

    counter
}
