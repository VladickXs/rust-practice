fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    let shift = ((n % len) + len) % len; // Нормалізація зсуву
    let (left, right) = s.split_at((len - shift) as usize);
    
    format!("{}{}", right, left)
}

#[test]
fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(rotate(s.clone(), *n), exp.to_string());
        });
}

fn main() {
    let s = "abcdefgh".to_string();
    let n = 2;
    println!("Результат: {}", rotate(s, n));
}
