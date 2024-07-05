use std::io;

fn main() {
    // Read the value of n (number of integers)
    println!("Enter the value of n:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid integer");

    // Initialize the sum
    let mut sum = 0;

    // Read n integers and add them to the sum
    for i in 0..n {
        println!("Enter integer {}: ", i + 1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please enter a valid integer");
        sum += num;
    }

    // Print the total sum
    println!("The sum of {} integers is: {}", n, sum);
}
