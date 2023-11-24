struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}
