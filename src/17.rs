// Define a struct for our custom type
struct MyCustomType {
    value: i32,
}

// Implement an equality operator for our custom type
impl PartialEq for MyCustomType {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

// Create a function to create instances of the custom type
fn create_custom_type() -> MyCustomType {
    MyCustomType { value: 42 }
}
