// Euler problem 2 - Sum all odd terms of the Fibonacci sequence up to a given limit
pub fn sum_fibonacci(max: uint) -> uint {
  let mut sum = 0u;
  let mut last_term = 2;
  let mut trailing_term = 1;
  let mut add_term = true;

  loop {
    let next_term = last_term + trailing_term;
    if add_term {
      sum += last_term;
    }
    
    if next_term > max {
      break;
    }

    trailing_term = last_term;
    last_term = next_term;
    add_term = !add_term;
  }

  sum
}