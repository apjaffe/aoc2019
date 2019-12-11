use std::io::{self, BufRead};
use std::collections::HashMap;

fn count_downstream(orbits:&HashMap<String,String>, cache:&mut HashMap<String,i64>, query:&str) -> i64 {
    if let Some(x) = cache.get(query) {
        return *x;
    }
    match orbits.get(query) {
        None => 0,
        Some(x) => {
            let res = count_downstream(orbits, cache, x) + 1;
            cache.insert(query.to_string(), res);
            res
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut orbits = HashMap::new();
    let mut cache = HashMap::new();
    for line in stdin.lock().lines() {
      let unwrapped = line.unwrap();
      let row = unwrapped.split(")").collect::<Vec<&str>>();
      orbits.insert(row[1].to_string(), row[0].to_string());
    }
    let mut count = 0;
    for key in orbits.keys() {
        count += count_downstream(&orbits, &mut cache, key);
    }

    println!("{}", count);
}
