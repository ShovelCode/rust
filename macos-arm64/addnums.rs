use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("nums.dat")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let number: i32 = line.trim().parse()?;
        sum += number;
    }

    println!("Sum of numbers: {}", sum);

    Ok(())
}
