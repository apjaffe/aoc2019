use std::io::{self, BufRead};
use std::collections::HashSet;

fn trace_wire(wire:Vec<&str>) -> HashSet<(i64,i64)> {
    let mut trace = HashSet::new();
    let mut x = 0;
    let mut y = 0;
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
            trace.insert((x,y));
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
    let intersect = trace1.intersection(&trace2); 
    let ans = intersect.map(|(x,y)| x.abs()+y.abs()).filter(|x| *x>0).min().unwrap();
    println!("{}", ans);
}
