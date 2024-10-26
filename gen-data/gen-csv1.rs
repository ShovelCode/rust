use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::error::Error;

const DATA_SIZE: usize = 1_000_000; // Adjust this number as needed

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut file = File::create("large_data.csv")?;
    
    // Write CSV header
    writeln!(file, "id,value1,value2,value3")?;
    
    for id in 1..=DATA_SIZE {
        let value1: f64 = rng.gen_range(0.0..100.0);
        let value2: f64 = rng.gen_range(0.0..100.0);
        let value3: f64 = rng.gen_range(0.0..100.0);
        
        writeln!(file, "{},{},{},{}", id, value1, value2, value3)?;
    }
    
    println!("Data generation complete! {} rows created.", DATA_SIZE);
    Ok(())
}
