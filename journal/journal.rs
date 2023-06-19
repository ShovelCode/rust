use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;
use chrono::{Local, Datelike, Timelike};

fn main() -> io::Result<()> {
    let mut entries = Vec::new();

    loop {
        let mut input = String::new();
        println!("Please enter your journal entry:");
        let bytes_read = BufReader::new(io::stdin()).read_line(&mut input)?;

        // If user didn't input anything (just hit return), break loop
        if bytes_read == 0 {
            break;
        }

        entries.push(input);
    }

    // Get the current date
    let now = Local::now();

    // Create a file with current date as name
    let filename = format!("{}_{}_{}.txt", now.year(), now.month(), now.day());
    let path = Path::new(&filename);
    let mut file = File::create(&path)?;

    // Write all journal entries to the file
    for entry in entries {
        writeln!(file, "{}", entry)?;
    }

    Ok(())
}
