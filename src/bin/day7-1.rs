extern crate aoc;

use std::io::{self,Read};
use std::collections::HashSet;
use aoc::Computer;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let amp_count:usize = 5;
    let phase_count:u32 = 5;
    let mut amps_orig = Vec::new();
    for _i in 0..amp_count {
        amps_orig.push(Computer::new_adv(&buffer, false));
    }
    let mut best = 0;
    for phases in 0..(phase_count.pow(amp_count as u32)) {
        let mut used = HashSet::new();
        let mut skip = false;
        for amp in 0..amp_count {
            let phase = (phases / (phase_count.pow(amp as u32))) % phase_count;
            if used.contains(&phase) {
                skip = true;
            }
            used.insert(phase);
        }
        if skip {
            continue;
        }

        let mut amps = amps_orig.clone();
        for amp in 0..amp_count {
            let phase = (phases / (phase_count.pow(amp as u32))) % phase_count;
            amps[amp].input(phase as i64);
        }
        for amp in 0..amp_count {
            if amp == 0 {
                amps[amp].input(0);
            } else {
                let out = amps[amp-1].output().unwrap();
                amps[amp].input(out);
            }
            amps[amp].run();
        }
        let res = amps[amp_count-1].output().unwrap();
        if res > best {
            println!("{}", phases);
        }
        best = best.max(res);
    }
    println!("{}", best);
}
