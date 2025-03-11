// Random Rust Code
use std::{thread, time};

fn main() {
    let now = time::Instant::now();
    thread::sleep(time::Duration::from_secs(5));
    println!("{}", now.elapsed().as_millis());
}
