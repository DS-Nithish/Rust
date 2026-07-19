struct TOdo {
    morning: String,
    afternoon: String,
    evening: String,
    night: String,
}

impl TOdo {
    fn new(&self) -> String {
        self.morning = "Good Morning".to_string()
    }
}

fn main() {
    println!("Hello World!");
}
