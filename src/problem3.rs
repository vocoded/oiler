use std::num::Float;

// Euler problem 3 - Find the largest prime factor of a given number (in this case, 600851475143)

// Brute force method; iterate downward from the halfway point
pub fn largest_prime_factor(num: uint) -> uint {
  let max = ((num / 2) + 1) as uint;

  range(2,max)
    .rev()
    .filter(|&x| { is_factor(num, x as uint) && is_prime(x as uint) })
    .take(1).next().unwrap()
}

// Slightly more efficient method; compute primes up to n^.5
pub fn largest_prime_factor_quick(num: uint) -> uint {
  let mut result = 0u;
  let mut term = 2;
  let max = (num as f64).sqrt() as uint;
  let primes = &mut vec![term];

  while term <= max {
    if is_factor(num, term) {
      result = term;
    }
    term = next_prime(primes);
  }

  if result == 0u { num } else { result }
}

fn is_factor(num: uint, factor: uint) -> bool {
  num % factor == 0
}

fn is_prime(num: uint) -> bool {
  let max = (num as f64).sqrt() as int;

  !range(2,max+1).any(|x| { is_factor(num as uint, x as uint) })
}

fn next_prime(primes: &mut Vec<uint>) -> uint {
  let mut term = primes[primes.len()-1];

  loop {
    term += 1;
    if !primes.iter().any(|p| { is_factor(term, *p) }) {
      primes.push(term);
      break;
    }
  }

  term
}