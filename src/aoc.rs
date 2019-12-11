#[crate_id = "aoc"]
#[crate_type = "lib"]

#[derive(Clone)]
pub struct Computer {
    pos: usize,
    mem: Vec<i64>
}

impl Computer {
    pub fn new(buffer:&str) -> Computer {
        Computer {
            pos: 0,
            mem: buffer.split(",")
                       .map(|s| s.trim().parse::<i64>().unwrap())
                       .collect::<Vec<i64>>()
        }
    }
    pub fn read(&mut self, pos: usize) -> i64 {
        return self.mem[pos];
    }
    pub fn write(&mut self, pos: usize, val:i64) {
        self.mem[pos] = val;
    }
    pub fn binop<F>(&mut self, f: F) where F: Fn(i64,i64)->i64 {
        let (a, b, c) = (self.mem[self.pos+1],
                         self.mem[self.pos+2],
                         self.mem[self.pos+3]);
        let res = f(self.mem[a as usize], self.mem[b as usize]);
        self.mem[c as usize] = res;
        self.pos += 4;
    }
    pub fn run(&mut self) {
        while self.step() {}
    }
    pub fn step(&mut self) -> bool {
        let instr = self.mem[self.pos];
        match instr {
            99 => false,
            1 => {
                self.binop(|x,y| x+y);
                true
            },
            2 => {
                self.binop(|x,y| x*y);
                true
            },
            _ => false
        }
    }
}
