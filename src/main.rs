extern crate problems;

use problems::problem1::{sum_of_factors_imperative, sum_of_factors_functional};
use problems::problem2::sum_fibonacci;

fn main() {
  let max = 999u;
  let sum_imperative = sum_of_factors_imperative(max);
  let sum_functional = sum_of_factors_functional(max);

  assert!(sum_imperative == sum_functional, "Sums differ");
  println!("The sum of factors of {} is {}", max, sum_imperative);

  let fib_max = 50;
  let fib_sum = sum_fibonacci(fib_max);
  println!("The sum of odd fib terms to {} is {}", fib_max, fib_sum);
}