use std::io::{self, BufRead};
use std::collections::HashMap;

fn downstream(orbits:&HashMap<String,String>, query:&str) -> HashMap<String, i64> {
    let mut current = query;
    let mut res = HashMap::new();
    let mut dist = 0;
    while let Some(next) = orbits.get(current) {
        res.insert(next.to_string(), dist);
        current = next;
        dist += 1;
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut orbits = HashMap::new();
    for line in stdin.lock().lines() {
      let unwrapped = line.unwrap();
      let row = unwrapped.split(")").collect::<Vec<&str>>();
      orbits.insert(row[1].to_string(), row[0].to_string());
    }
    let down1 = downstream(&orbits, "YOU");
    let down2 = downstream(&orbits, "SAN");
    let mut best = -1;
    for (key, dist1) in down1.iter() {
       if let Some(dist2) = down2.get(key) {
           if best == -1 || best > dist1 + dist2 {
               best = dist1 + dist2;
            }
       }
    }

    println!("{}", best);
}
