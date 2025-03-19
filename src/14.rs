fn main() {
    let mut n = 1;
    while n < 100 {
        if n % 2 == 0 {
            println!("Even: {}", n);
        } else {
            println!("Odd: {}", n);
        }
        n += 1;
    }
}
