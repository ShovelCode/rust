fn rot13(s: &str) -> String {
    s.chars()
        .map(|c| {
            match c {
                'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
                'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
                _ => c,
            }
        })
        .collect()
}

fn main() {
    let input = "Hello, World!";
    let output = rot13(input);
    println!("The Rot13 of {} is {}", input, output);
}
