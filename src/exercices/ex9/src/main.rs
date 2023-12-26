static PARAMETER: i32 = 1400;

fn main() {
    let s2 = PARAMETER / 2;
    let mlimit = (s2 as f64).sqrt() as i32 - 1;

    for m in 2..mlimit {
        if s2 % m == 0 {
            let mut sm = s2 / m;
            while sm % 2 == 0 {
                sm /= 2;
            }

            let mut k: i32;
            if m % 2 == 1 {
                k = m + 2;
            } else {
                k = m + 1;
            }

            while k < 2 * m && k <= sm {
                if sm % k == 0 && gcd(&k, &m) == 1 {
                    let d = s2 / (k * m);
                    let n = k - m;
                    let a = d * (m * m - n * n);
                    let b = 2 * d * m * n;
                    let c = d * (m * m + n * n);
                    println!("a = {}, b = {}, c = {}", a, b, c);
                    println!("the product of these values is {}\n", a * b * c);
                }
                k += 2;
            }
        }
    }
}

fn gcd(k: &i32, m: &i32) -> i32 {
    if m == &0 {
        return *k;
    }
    gcd(m, &(k % m))
}
