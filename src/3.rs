use std::collections::HashMap;

fn main() {
    let mut numbers = HashMap::new();
    numbers.insert(1, 2);
    numbers.insert(3, 4);

    for (key, value) in &numbers {
        println!("{}: {}", key, value);
    }
}
