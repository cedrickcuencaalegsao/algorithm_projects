fn main() {
  let mut s: i32 = 0; // Initialize sum to zero
  let mut i: i32 = 0; // Initialize counter to zero
  let mut n = String::new(); // Declare n as a mutable String

  println!("Enter the value of N:");
  std::io::stdin().read_line(&mut n).expect("Failed to read line"); // Read N as a string

  // Convert the string input to an integer
  let n: i32 = n.trim().parse().expect("Please enter a valid integer!");

  while i < n {
      let mut a_string = String::new(); // Temporary string for input
      println!("Enter number:");
      std::io::stdin().read_line(&mut a_string).expect("Failed to read line");
      let a: i32 = a_string.trim().parse().expect("Please type a number!"); // Convert string input to integer

      s += a; // Add current number to sum
      i += 1; // Increment counter
  }

  println!("The sum of given numbers is: {}", s);
}
