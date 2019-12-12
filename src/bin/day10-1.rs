use std::io::{self, BufRead};
use std::collections::HashSet;

// https://en.wikipedia.org/wiki/Euclidean_algorithm#Implementations
fn calc_gcd(a:i64, b:i64) -> i64 {
  if b == 0 {
    a
  } else {
    calc_gcd(b, a % b)
  }
}

fn has_sight(coords:&HashSet<(i64,i64)>,coord1:&(i64,i64),coord2:&(i64,i64)) -> bool {
  if coord1 == coord2 {
    return false;
  }
  let (x1, y1) = coord1;
  let (x2, y2) = coord2;
  let dx = x2-x1;
  let dy = y2-y1;
  let gcd = calc_gcd(dx.abs(),dy.abs());
  let dx = dx / gcd;
  let dy = dy / gcd;
  let mut x = x1 + dx;
  let mut y = y1 + dy;
  while x != *x2 || y != *y2 {
    if coords.contains(&(x, y)) {
      return false;
    }
    x += dx;
    y += dy;
  }
  true
}

fn main() {
    let stdin = io::stdin();
    let mut coords = HashSet::new();
    for (y, line) in stdin.lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
          if c == '#' {
            coords.insert((x as i64,y as i64));
          }
        }
    }
    let best = coords.iter().map(|coord1| coords.iter().filter(|coord2| has_sight(&coords, &coord1, &coord2)).count()).max().unwrap();
    println!("{}", best);
}
