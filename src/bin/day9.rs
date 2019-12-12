extern crate aoc;

use std::io::{self,Read};
use aoc::Computer;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();
    let mut comp = Computer::new_adv(&buffer, true);

    comp.input(2);
    comp.run();
}
