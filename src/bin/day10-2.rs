
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::f64;

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

fn calc_angle(x1:i64,y1:i64,x2:i64,y2:i64) -> i64 {
    let angle = ((y2-y1) as f64).atan2((x2-x1) as f64);
    let angle = if angle < -f64::consts::FRAC_PI_2 {
        angle + f64::consts::PI * 2.0_f64
    } else {
        angle
    };
    (angle * 1e15) as i64
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
    let mut best_k = (0,0);
    let mut best_v = 0;
    for coord1 in coords.iter() {
        let count = coords.iter().filter(|coord2| has_sight(&coords, &coord1, &coord2)).count();
        if count > best_v {
            best_k = *coord1;
            best_v = count;
        }
    }
    println!("{:?}, {:?}", best_k, best_v);

    let (x1, y1) = best_k;
    let mut vaporized = coords.iter().filter(|coord2| has_sight(&coords, &best_k, &coord2)).collect::<Vec<&(i64,i64)>>();
    vaporized.sort_by_key(|(x2,y2)| calc_angle(x1,y1,*x2,*y2));
    println!("{:?}", vaporized);
    println!("{:?}", vaporized[199]);
    let (ax, ay) = vaporized[199];
    println!("{}", (ax*100+ay));

}
