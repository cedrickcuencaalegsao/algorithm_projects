fn factorial(n: u64) -> u64 {
  if n == 0 {
      return 1;
  }

  let mut result = 1;

  for i in 2..=n {
      result *= i;
  }

  result
}

fn main() {
  let n: u64 = 5; 

  let result = factorial(n);

  println!("Factorial of {} is {}", n, result);
}
