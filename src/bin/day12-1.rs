#[derive(Clone,Debug)]
struct Moon {
  x: i64,
  y: i64,
  z: i64,
  dx: i64,
  dy: i64,
  dz: i64
}

impl Moon {
  fn new(x:i64, y:i64, z:i64) -> Moon {
    Moon {
      x: x,
      y: y,
      z: z,
      dx: 0,
      dy: 0,
      dz: 0
    }
  }
  fn gravity(self: &mut Moon, other: &Moon) {
    if other.x > self.x {
      self.dx += 1
    }
    if other.x < self.x {
      self.dx -= 1;
    }
    if other.y > self.y {
      self.dy += 1
    }
    if other.y < self.y {
      self.dy -= 1;
    }
    if other.z > self.z {
      self.dz += 1
    }
    if other.z < self.z {
      self.dz -= 1;
    }
  }
  fn velocity(self: &mut Moon) {
    self.x += self.dx;
    self.y += self.dy;
    self.z += self.dz;
  }
  fn energy(self: &Moon) -> i64 {
    let pot = self.x.abs() + self.y.abs() + self.z.abs();
    let kin = self.dx.abs() + self.dy.abs() + self.dz.abs();
    pot*kin
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

fn main() {
  let mut moons = Vec::new();
  /*moons.push(Moon::new(-1,0,2));
  moons.push(Moon::new(2,-10,-7));
  moons.push(Moon::new(4,-8,8));
  moons.push(Moon::new(3,5,-1));*/
  moons.push(Moon::new(3,15,8));
  moons.push(Moon::new(5,-1,-2));
  moons.push(Moon::new(-10,8,2));
  moons.push(Moon::new(8,4,-5));
  for _i in 0..1000 {
    step(&mut moons);
  }
  let energy:i64 = moons.iter().map(|m|m.energy()).sum();
  println!("{}",energy);
}
