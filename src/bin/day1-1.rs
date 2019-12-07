use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut sum : i64 = 0;
    for line in stdin.lock().lines() {
      let val : i64 = line.unwrap().parse::<i64>().unwrap();
      sum += (val / 3) - 2;
    }
    println!("{}", sum);
}
