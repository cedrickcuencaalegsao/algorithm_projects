fn main() {
  println!("An algorithm should be simple.");

  println!("An algorithm should be clear and unambiguous.");

  println!("An algorithm should lead to a unique solution.");

  println!("An algorithm involves a finite number of steps.");

  let a = 10;
  let b = 0;
  if b != 0 {
      let result = a / b;
      println!("Result of division: {}", result);
  } else {
      println!("Error: Division by zero!");
  }
}
