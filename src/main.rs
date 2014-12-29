// Euler problem 1 - Sum all natural numbers less than 1000 divisible by 3 or 5 

// Generate result from standard iterator over a range
fn sum_of_factors_imperative(max: uint) -> uint {
  let mut sum = 0u;
  for i in range(1, max) {
    if i % 3 == 0 || i % 5 == 0 {
      sum += i;
    }
  }
  sum
}

// Generate result with a more functional style
fn sum_of_factors_functional(max: uint) -> uint {
  range(1, max).filter(|&i| i % 3 == 0 || i % 5 == 0).fold(0, |i, s| { i + s })
}

fn main() {
  let max = 999u;
  let sum_imperative = sum_of_factors_imperative(max);
  let sum_functional = sum_of_factors_functional(max);

  assert!(sum_imperative == sum_functional, "Sums differ");
  println!("The sum of factors of {} is {}", max, sum_imperative);
}