enum MyData {
    Integer(i32),
    Float(f64),
    Text(String),
}

fn first_element(array: &[MyData]) -> Option<&MyData> {
    array.first()
}

fn main() {
    let array = vec![
        MyData::Integer(42),
        MyData::Float(3.14),
        MyData::Text(String::from("Hello")),
    ];

    if let Some(first) = first_element(&array) {
        match first {
            MyData::Integer(i) => println!("First element is an integer: {}", i),
            MyData::Float(f) => println!("First element is a float: {}", f),
            MyData::Text(t) => println!("First element is a text: {}", t),
        }
    } else {
        println!("Array is empty");
    }
}
