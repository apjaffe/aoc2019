use std::collections::HashSet;

#[derive(Clone,Debug,Hash,PartialEq,Eq)]
struct Moon {
  x: i64,
  dx: i64
}

impl Moon {
  fn new(x:i64) -> Moon {
    Moon {
      x: x,
      dx: 0
    }
  }
  fn gravity(self: &mut Moon, other: &Moon) {
    if other.x > self.x {
      self.dx += 1
    }
    if other.x < self.x {
      self.dx -= 1;
    }
  }
  fn velocity(self: &mut Moon) {
    self.x += self.dx;
  }
}

fn step(moons: &mut Vec<Moon>) {
  let moon_copy = moons.clone();
  for i in 0..moons.len() {
    for j in 0..moons.len() {
      if i != j {
        moons[i].gravity(&moon_copy[j]);
      }
    }
  }
  for moon in moons {
    moon.velocity();
  }
}

fn run_to_repeat(mut moons: Vec<Moon>) -> i64 {
  let mut states = HashSet::new();
  let mut count = 0;
  loop {
    step(&mut moons);
    if states.contains(&moons) {
      return count;
    }
    count += 1;
    states.insert(moons.clone());
  }
}

fn count_repeat(coords: Vec<i64>) -> i64 {
  let mut moons = Vec::new();
  for x in &coords {
    moons.push(Moon::new(*x));
  }
  run_to_repeat(moons)
}

// https://en.wikipedia.org/wiki/Euclidean_algorithm#Implementations
fn calc_gcd(a:i64, b:i64) -> i64 {
  if b == 0 {
    a
  } else {
    calc_gcd(b, a % b)
  }
}

fn main() {
  let mut repeats = Vec::new();
  //repeats.push(count_repeat([-1,2,4,3].to_vec()));
  //repeats.push(count_repeat([0,-10,-8,5].to_vec()));
  //repeats.push(count_repeat([2,-7,8,-1].to_vec()));
  //repeats.push(count_repeat([-8,5,2,9].to_vec()));
  //repeats.push(count_repeat([-10,5,-7,-8].to_vec()));
  //repeats.push(count_repeat([0,10,3,-3].to_vec()));
  repeats.push(count_repeat([3,5,-10,8].to_vec()));
  repeats.push(count_repeat([15,-1,8,4].to_vec()));
  repeats.push(count_repeat([8,-2,2,-5].to_vec()));
  println!("{:?}", repeats);
  let mut total = repeats.iter().fold(1, |accum, i| accum * i / calc_gcd(accum, *i));
  println!("{}", total);
}
