fn draw_tree(levels: usize) {
    (0..levels).for_each(|level| {
        let height = level + 3; // Висота кожного трикутника збільшується
        (0..height).for_each(|row| {
            let spaces = levels + 2 - row; // Відступи для центрування
            let stars = 2 * row + 1;
            println!("{:width$}{}", "", "*".repeat(stars), width = spaces);
        });
    });

    // Малюємо стовбур
    let trunk_width = 3;
    let trunk_height = 2;
    let spaces = levels + 1;
    (0..trunk_height).for_each(|_| {
        println!("{:width$}{}", "", "*".repeat(trunk_width), width = spaces);
    });
}

fn main() {
    let levels = 5; // Кількість трикутників
    draw_tree(levels);
}
