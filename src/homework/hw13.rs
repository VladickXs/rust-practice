#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    // Обчислення площі прямокутника
    fn area(&self) -> i32 {
        (self.b.x - self.a.x).abs() * (self.a.y - self.b.y).abs()
    }

    // Обчислення площі перекриття між двома прямокутниками
    fn overlap(&self, other: &Rectangle) -> i32 {
        let x_overlap = (self.b.x.min(other.b.x) - self.a.x.max(other.a.x)).max(0);
        let y_overlap = (self.a.y.min(other.a.y) - self.b.y.max(other.b.y)).max(0);
        x_overlap * y_overlap
    }
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;

    // Спочатку обчислюємо суму площ усіх прямокутників
    for i in 0..xs.len() {
        total_area += xs[i].area();
    }

    // Тепер віднімаємо площі перекриття
    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            total_area -= xs[i].overlap(&xs[j]);
        }
    }

    total_area
}

fn main() {
    // Тестові дані
    let data = vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ];

    // Обчислення фактично зайнятої площі
    let occupied = area_occupied(&data);
    
    // Виведення результату
    println!("Occupied area: {}", occupied);
}
