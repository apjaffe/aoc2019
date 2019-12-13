extern crate aoc;

use std::io::{self,Read};
use std::collections::HashMap;
use aoc::Computer;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();
    let mut comp = Computer::new_adv(&buffer, false);
    let mut screen = HashMap::new();

    comp.run();
    while let (Some(x), Some(y), Some(tile)) =
      (comp.output(), comp.output(), comp.output()) {
      screen.insert((x,y), tile);
    }
    println!("{}", screen.values().filter(|x| **x == 2).count());
}
