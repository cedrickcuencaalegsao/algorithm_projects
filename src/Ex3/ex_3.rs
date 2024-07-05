use std::io;

fn main() {

    let mut largest = 0.0; 
    let mut count = 0; 

    while count < 10 {
        count += 1; 

        println!("Enter number {}:", count);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let x: f64 = input
            .trim()
            .parse()
            .expect("Invalid input. Please enter a valid number.");

        if x > largest {
            largest = x;
        }
    }

    println!("Largest number: {}", largest);
}
