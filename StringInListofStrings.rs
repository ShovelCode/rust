fn contains_string(list: &[String], query: &str) -> bool {
    list.iter().any(|s| s == query)
}

fn main() {
    let strings = vec![
        "apple".to_string(),
        "banana".to_string(),
        "orange".to_string(),
    ];

    let query = "banana";
    let exists = contains_string(&strings, query);

    if exists {
        println!("'{}' exists in the list.", query);
    } else {
        println!("'{}' does not exist in the list.", query);
    }
}
