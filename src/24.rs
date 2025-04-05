// Rust program to demonstrate basic operations with vectors and arrays.
use std::vec::Vec;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Original vector: {:?}", numbers);
    
    // Adding elements
    let sum_of_elements = numbers.iter().sum();
    println!("Sum of all elements: {}", sum_of_elements);
    
    // Shaping a vector into an array
    let shaped_array = [10, 20, 30];
    println!("Shaped array: {:?}", shaped_array);
    
    // Sorting the array using Rust's built-in sort function
    let sorted_numbers = numbers.sort();
    println!("Sorted numbers: {:?}", sorted_numbers);
}
