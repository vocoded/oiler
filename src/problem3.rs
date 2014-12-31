use std::num::Float;

// Euler problem 3 - Find the largest prime factor of a given number (in this case, 600851475143)

pub fn largest_prime_factor(num: uint) -> uint {
  let max = ((num / 2) + 1) as uint;

  range(2,max)
    .rev()
    .filter(|&x| { is_factor(num, x as uint) && is_prime(x as uint) })
    .take(1).next().unwrap()
}

fn is_factor(num: uint, factor: uint) -> bool {
  num % factor == 0
}

fn is_prime(num: uint) -> bool {
  let max = (num as f64).sqrt() as int;

  !range(2,max+1).any(|x| { is_factor(num as uint, x as uint) })
}