fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect()
}

#[test]
fn test() {
    let data = [
        ("Hello", "hELLO"),
        ("Привет", "пРИВЕТ"),
        ("Rust", "rUST"),
        ("TeSt", "tEsT"),
    ];

    data.iter().for_each(|(a, b)| {
        assert_eq!(invert_the_case(a.to_string()), b.to_string());
        assert_eq!(invert_the_case(b.to_string()), a.to_string());
    });
}

fn main() {
    let input = "Hello, Rust!";
    let output = invert_the_case(input.to_string());
    println!("{}", output); // hELLO, rUST!
}
