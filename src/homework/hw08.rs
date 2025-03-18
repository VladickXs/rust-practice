fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[test]
fn test() {
    let data = [
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (10, false),
        (13, true),
        (17, true),
        (19, true),
        (20, false),
    ];

    data.iter().for_each(|(num, expected)| {
        assert_eq!(is_prime(*num), *expected);
    });
}

fn main() {
    let num = 29;
    println!("Число {} є простим: {}", num, is_prime(num));
}
