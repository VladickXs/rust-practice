fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    // Перевіряємо, чи можливо рівномірно розподілити вантажі
    if total % n as u32 != 0 {
        return 0; // Рівномірний розподіл неможливий
    }

    let average = total / n as u32;
    let mut moves = 0;
    let mut balance = 0;

    // Підраховуємо мінімальні переміщення
    for &shipment in shipments.iter() {
        balance += shipment as i32 - average as i32;
        moves += balance.abs();
    }

    moves as usize
}

fn is_possible_equal_distribution(shipments: &Vec<u32>) -> bool {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();
    
    // Перевірка, чи можливо рівномірно розподілити вантажі
    total % n as u32 == 0
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = Vec::with_capacity(n);
    let average = 10; // Наприклад, кожен корабель має отримати 10 одиниць вантажу
    let mut total = average * n as u32;
    
    for _ in 0..n {
        shipments.push(average);
    }

    shipments
}

fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    let shipments2 = vec![9, 3, 7, 2, 9];

    // Перевіряємо можливість рівномірного розподілу
    println!("Is possible for shipments1: {}", is_possible_equal_distribution(&shipments1)); // true
    println!("Is possible for shipments2: {}", is_possible_equal_distribution(&shipments2)); // true

    // Підрахунок мінімальної кількості переміщень вантажу
    let moves1 = count_permutation(&shipments1);
    let moves2 = count_permutation(&shipments2);

    println!("Minimum moves for shipments1: {}", moves1); // 4
    println!("Minimum moves for shipments2: {}", moves2); // 7
}
