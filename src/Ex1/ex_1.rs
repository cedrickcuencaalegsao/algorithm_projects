use std::io;

fn main() {
    //   Ex. 1. Write the algorithm to find the sum and product of two given
    // numbers.

    // Algorithm: To find the sum and product of two given numbers:
    // Step 1: Read A , B
    // Step 2: Let Sum= A+B
    // Step 3: Let Product=A*B
    // Step 4: Print Sum, Product
    // Step 5: Stop.

    // Read input from the user
    println!("Enter the first number (A):");
    let mut input_a = String::new();
    io::stdin()
        .read_line(&mut input_a)
        .expect("Failed to read input");
    let a: f64 = input_a
        .trim()
        .parse()
        .expect("Invalid input. Please enter a valid number.");

    println!("Enter the second number (B):");
    let mut input_b = String::new();
    io::stdin()
        .read_line(&mut input_b)
        .expect("Failed to read input");
    let b: f64 = input_b
        .trim()
        .parse()
        .expect("Invalid input. Please enter a valid number.");

    // Calculate sum and product
    let sum = a + b;
    let product = a * b;

    // Print the results
    println!("Sum: {}", sum);
    println!("Product: {}", product);
}
