use std::io::{self, BufRead};
use std::collections::{HashMap,HashSet};

fn trace_wire(wire:Vec<&str>) -> HashMap<(i64,i64),i64> {
    let mut trace = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut ln = 0;
    for w in wire {
        let (xd, yd) = match w.chars().nth(0).unwrap() {
            'R' => (1,0),
            'D' => (0,1),
            'U' => (0,-1),
            'L' => (-1,0),
            _ => panic!("bad input")
        };
        let len = w.split_at(1).1.parse::<i64>().unwrap();
        for i in 0..len {
            x += xd;
            y += yd;
            ln += 1;
            trace.entry((x,y)).or_insert(ln);
        }
    }
    trace
}

fn main() {
    let stdin = io::stdin();
    let mut buffer1 = String::new();
    let mut handle = stdin.lock();
    handle.read_line(&mut buffer1).unwrap();
    let wire1 = buffer1.trim().split(",").collect::<Vec<&str>>();
    let mut buffer2 = String::new();
    handle.read_line(&mut buffer2).unwrap();
    let wire2 = buffer2.trim().split(",").collect::<Vec<&str>>();
    let trace1 = trace_wire(wire1);
    let trace2 = trace_wire(wire2);
    // sure you could do this faster (i.e. check each pair of lines)
    // but this is fast enough...
    let keys1 =  trace1.keys().collect::<HashSet<&(i64,i64)>>();
    let keys2 =  trace2.keys().collect::<HashSet<&(i64,i64)>>();
    let intersect = keys1.intersection(&keys2);
    let ans = intersect.map(|coord| trace1[coord] + trace2[coord]).filter(|x| *x>0).min().unwrap();
    println!("{}", ans);
}
