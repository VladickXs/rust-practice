use std::time::{SystemTime, UNIX_EPOCH};

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut vec = Vec::with_capacity(n);

    for _ in 0..n {
        rng = rng.wrapping_add(1);
        vec.push(((rng % 90) + 10) as i32);  // Генеруємо числа від 10 до 99
    }

    vec
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_indexes = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indexes = (i, i + 1);
        }
    }

    (min_sum, min_indexes.0, min_indexes.1)
}

fn print_vector_with_highlight(data: &[i32], min_sum: i32, idx1: usize, idx2: usize) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:3} ", i);
    }
    println!();

    print!("data:    ");
    for i in 0..data.len() {
        if i == idx1 || i == idx2 {
            print!("\x1b[31m{:3} \x1b[0m", data[i]); // Виділяємо червоним
        } else {
            print!("{:3} ", data[i]);
        }
    }
    println!();

    println!("min adjacent sum={}+{}={} at indexes:{},{}\n", data[idx1], data[idx2], min_sum, idx1, idx2);
}

fn main() {
    // Генерація випадкових векторів
    let vec1 = gen_random_vector(20);
    let vec2 = gen_random_vector(20);
    let vec3 = gen_random_vector(20);
    let vec4 = gen_random_vector(20);

    // Знаходимо мінімальні пари для кожного вектора
    let (sum1, idx1, idx2) = min_adjacent_sum(&vec1);
    let (sum2, idx3, idx4) = min_adjacent_sum(&vec2);
    let (sum3, idx5, idx6) = min_adjacent_sum(&vec3);
    let (sum4, idx7, idx8) = min_adjacent_sum(&vec4);

    // Виведення результатів для кожного випадкового вектора
    print_vector_with_highlight(&vec1, sum1, idx1, idx2);
    print_vector_with_highlight(&vec2, sum2, idx3, idx4);
    print_vector_with_highlight(&vec3, sum3, idx5, idx6);
    print_vector_with_highlight(&vec4, sum4, idx7, idx8);
}
