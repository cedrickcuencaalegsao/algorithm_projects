use std::io;

fn main() {
    //   Ex.3: Write an algorithm to find the largest number from 10 given
    // numbers.
    // Algorithm: To find the largest number from 10 given numbers.
    // 1. [Initialize the variables]
    // large = 0
    // count = 0
    // 2. [Increment the count]
    // count = count+1
    // 3. [Enter number]
    // read x
    // 4. [Perform the operations]
    // if (t>x) then
    // goto step 6
    // end if
    // 5. large = x
    // 6. if (count  10) then
    // goto step 2
    // end if
    // 7. [Print the result]
    // Print large
    // 8. [Finished]
    // Stop
    
    let mut largest = 0.0; // Initialize the largest variable
    let mut count = 0; // Initialize the count variable

    while count < 10 {
        count += 1; // Increment the count

        // Read input from the user
        println!("Enter number {}:", count);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let x: f64 = input
            .trim()
            .parse()
            .expect("Invalid input. Please enter a valid number.");

        // Update the largest number if necessary
        if x > largest {
            largest = x;
        }
    }

    // Print the largest number
    println!("Largest number: {}", largest);
}
