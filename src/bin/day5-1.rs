extern crate aoc;

use std::io::{self,Read};
use aoc::Computer;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();
    let mut comp = Computer::new(&buffer);

    for noun in 0..99 {
        for verb in 0..99 {
            let mut comp_cp = comp.clone();
            comp_cp.write(1, noun);
            comp_cp.write(2, verb);
            comp_cp.run();
            if comp_cp.read(0) == 19690720 {
                println!("{}", 100*noun+verb);
            }
        }
    }
}
