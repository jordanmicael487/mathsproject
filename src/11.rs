// Generate a random number between 1 and 10
let mut rng = rand::thread_rng();
let random_number = rng.gen_range(1, 11);

// Print the random number
println!("The random number is {}", random_number);
