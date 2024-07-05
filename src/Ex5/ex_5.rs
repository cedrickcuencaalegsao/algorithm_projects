use std::io;

fn main() {
    // Read input from the user
    println!("Enter the first number (A):");
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("Failed to read input");
    let a: f64 = input_a.trim().parse().expect("Invalid input. Please enter a valid number.");

    println!("Enter the second number (B):");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("Failed to read input");
    let b: f64 = input_b.trim().parse().expect("Invalid input. Please enter a valid number.");

    // Calculate sum and product
    let sum = a + b;
    let product = a * b;

    // Print the results
    println!("Sum: {}", sum);
    println!("Product: {}", product);
}
