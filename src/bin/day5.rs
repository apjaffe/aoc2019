extern crate aoc;

use std::io::{self,Read};
use aoc::Computer;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();
    let mut comp = Computer::new(&buffer);

    comp.input(5);
    let mut comp_cp = comp.clone();
    comp.run();
}
