fn main() {
    const SIZE: usize = 5; // Висота половини ромба

    let mut output = String::new();

    // Верхня частина ромба
    for i in 0..SIZE {
        for j in 0..(SIZE - i - 1) {
            output.push(' '); // Пробіли перед зірочками
        }
        for j in 0..(2 * i + 1) {
            output.push('*'); // Зірочки
        }
        output.push('\n');
    }

    // Нижня частина ромба
    for i in (0..SIZE - 1).rev() {
        for j in 0..(SIZE - i - 1) {
            output.push(' '); // Пробіли перед зірочками
        }
        for j in 0..(2 * i + 1) {
            output.push('*'); // Зірочки
        }
        output.push('\n');
    }

    print!("{}", output);
}
