fn main() {
    const WIDTH: usize = 10;  // Ширина конверта
    const HEIGHT: usize = 5;  // Висота конверта

    let mut output = String::new();

    // Верхня частина (трикутник)
    for i in 0..HEIGHT {
        for j in 0..(WIDTH * 2 + 1) {
            if j == WIDTH - i || j == WIDTH + i {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    // Нижня частина (прямокутник)
    for i in 0..HEIGHT {
        for j in 0..(WIDTH * 2 + 1) {
            if j == 0 || j == WIDTH * 2 || i == HEIGHT - 1 {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
