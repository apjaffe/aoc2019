use std::io::{self, BufRead};

fn calc_fuel(mass : i64) -> i64 {
  if mass < 9 {
    return 0;
  }
  let f = (mass / 3) - 2;
  return f + calc_fuel(f);
}

fn main() {
    let stdin = io::stdin();
    let mut sum : i64 = 0;
    for line in stdin.lock().lines() {
      let val : i64 = line.unwrap().parse::<i64>().unwrap();
      sum += calc_fuel(val);
    }
    println!("{}", sum);
}
