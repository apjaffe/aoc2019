use std::io::{self,Read};

fn step_computer(pos:&mut usize, mem:&mut Vec<usize>) -> bool {
    match mem[*pos] {
        99 => false,
        1 => {
            let (a, b, c) = (mem[*pos+1], mem[*pos+2], mem[*pos+3]);
            let res = mem[a] + mem[b];
            mem[c] = res;
            *pos += 4;
            true
        },
        2 => {
            let (a, b, c) = (mem[*pos+1], mem[*pos+2], mem[*pos+3]);
            let res = mem[a] * mem[b];
            mem[c] = res;
            *pos += 4;
            true
        },
        _ => false
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();
    let mut mem = buffer.split(",").map(|s| s.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut pos = 0;

    mem[1] = 12;
    mem[2] = 2;
    while step_computer(&mut pos, &mut mem) {
    }
    println!("{:?}", mem);
}
