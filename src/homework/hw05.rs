fn gcd(a: u32, b: u32) -> u32 {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn main() {
    let a = 120;
    let b = 90;
    println!("НСД({}, {}) = {}", a, b, gcd(a, b));
}
