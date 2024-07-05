use std::io;

fn main() {
  
    println!("Enter the number of elements (N):");
    let mut input_n = String::new();
    io::stdin().read_line(&mut input_n).expect("Failed to read input");
    let n: usize = input_n.trim().parse().expect("Invalid input. Please enter a valid integer.");

    let mut sum = 0.0;
    for i in 1..=n {
        println!("Enter number {}:", i);
        let mut input_num = String::new();
        io::stdin().read_line(&mut input_num).expect("Failed to read input");
        let num: f64 = input_num.trim().parse().expect("Invalid input. Please enter a valid number.");
        sum += num;
    }

    let am = sum / n as f64;

    println!("Sum: {}", sum);
    println!("Arithmetic Mean: {}", am);
}
