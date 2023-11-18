use std::io;
use std::collections::HashMap;

fn main() {
    let mut numbers = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => break,
        }
    }

    numbers.sort();

    let sum: i32 = numbers.iter().sum();
    let mean = sum as f32 / numbers.len() as f32;

    let mid = numbers.len() / 2;
    let median = if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) as f32 / 2.0
    } else {
        numbers[mid] as f32
    };

    let mut map = HashMap::new();
    for num in &numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mode = map.into_iter().max_by_key(|&(_, count)| count).map(|(num, _)| *num);

    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {:?}", mode);
}
