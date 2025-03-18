fn find_variables() -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    // Це приклад значень для змінних m, u, x, a, s, l, o, n
    let m = 2;
    let u = 3;
    let x = 5;
    let a = 4;
    let s = 2;
    let l = 1;
    let o = 3;
    let n = 2;

    (m, u, x, a, s, l, o, n)
}

fn calculate_and_print() {
    let (m, u, x, a, s, l, o, n) = find_variables();

    // Множимо m, u, x, a
    let numerator = m * u * x * a;
    // Множимо s, l, o, n
    let denominator = s * l * o * n;

    // Виводимо результат у форматі:
    println!("{: <5}", format!("{}{}{}{}", m, u, x, a));
    println!("{: <5}", a);
    println!("{: <5}------", x);
    println!("{: <5}", format!("{}{}{}{}", s, l, o, n));

    // Рахуємо результат і виводимо
    let result = numerator / denominator;
    println!("Result: {}", result);
}

fn main() {
    calculate_and_print();
}
