use std::collections::HashMap;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut my_map: HashMap<&str, i32> = HashMap::new();

    for (i, &number) in numbers.iter().enumerate() {
        if number % 2 == 0 {
            my_map.insert(&format!("even_{}", i), number);
        }
    }

    println!("{:?}", my_map);
}
