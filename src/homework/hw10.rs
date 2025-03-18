fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev()) // Порівнюємо рядок із його реверсованою версією
}

#[test]
fn test_palindrome() {
    let cases = [
        (121, true),
        (12321, true),
        (10, false),
        (123, false),
        (444, true),
        (0, true),
    ];

    for (num, expected) in cases.iter() {
        assert_eq!(is_palindrome(*num), *expected);
    }
}

fn main() {
    let num = 12321;
    println!("Число {} є паліндромом: {}", num, is_palindrome(num));
}
