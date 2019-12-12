use std::collections::VecDeque;

#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum State {
    Running,
    WaitingInput,
    Halted
}

#[derive(Clone)]
pub struct Computer {
    pos: usize,
    mem: Vec<i64>,
    inputs: VecDeque<i64>,
    outputs: VecDeque<i64>,
    state: State,
    log: bool,
    rb: i64 // relative base
}

impl Computer {
    pub fn new(buffer:&str) -> Computer {
        Computer::new_adv(buffer, true)
    }
    pub fn new_adv(buffer:&str, log:bool) -> Computer {
        Computer {
            pos: 0,
            mem: buffer.split(",")
                       .map(|s| s.trim().parse::<i64>().unwrap())
                       .collect::<Vec<i64>>(),
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            state: State::Running,
            log: log,
            rb: 0
        }
    }
    pub fn input(&mut self, val:i64) {
        self.inputs.push_back(val);
    }
    pub fn output(&mut self) -> Option<i64> {
        self.outputs.pop_front()
    }
    pub fn read(&self, pos: usize) -> i64 {
        if pos >= self.mem.len() {
          return 0;
        }
        self.mem[pos]
    }
    pub fn write(&mut self, pos: usize, val:i64) {
        if pos >= self.mem.len() {
          self.mem.resize(pos+1, 0);
        }
        self.mem[pos] = val;
    }
    fn parse_mode(&self, instr: i64, param: u8) -> i64 {
        (instr / (10_i64.pow(2+param as u32))) % 10
    }
    fn read_mode(&self, instr: i64, param:u8) -> i64 {
        let arg = self.mem[self.pos + 1 + (param as usize)];
        match self.parse_mode(instr, param) {
            0 => self.read(arg as usize),
            1 => arg,
            2 => self.read((self.rb + arg) as usize),
            _ => panic!("Invalid mode")
        }
    }
    fn write_mode(&mut self, instr: i64, param:u8, val:i64) {
        let arg = self.mem[self.pos + 1 + (param as usize)];
        match self.parse_mode(instr, param) {
            0 => self.write(arg as usize, val),
            1 => panic!("Can't write to immediate"),
            2 => self.write((self.rb + arg) as usize, val),
            _ => panic!("Invalid mode")
        }
    }
    fn binop<F>(&mut self, instr: i64, f: F) where F: Fn(i64,i64)->i64 {
        let arg1 = self.read_mode(instr, 0);
        let arg2 = self.read_mode(instr, 1);
        let res = f(arg1, arg2);
        self.write_mode(instr, 2, res);
        self.pos += 4;
    }
    fn cond_jump_un<F>(&mut self, instr:i64, test:F) where F: Fn(i64)->bool {
        let arg1 = self.read_mode(instr, 0);
        let arg2 = self.read_mode(instr, 1);
        if test(arg1) {
            self.pos = arg2 as usize;
        } else {
            self.pos += 3;
        }
    }
    pub fn is_halted(&self) -> bool {
        State::Halted == self.state
    }
    pub fn run_until_waiting(&mut self) {
        while State::Running == self.step() {}
    }
    pub fn run(&mut self) {
        while State::Halted != self.step() {}
    }
    pub fn step(&mut self) -> State {
        let instr = self.mem[self.pos];
        match instr % 100 {
            // halt
            99 => {
                self.state = State::Halted
            }
            // add
            1 => self.binop(instr, |x,y| x+y),
            // mult
            2 => self.binop(instr, |x,y| x*y),
            // input
            3 => match self.inputs.pop_front() {
                None => self.state = State::WaitingInput,
                Some(inp) => {
                    self.write_mode(instr, 0, inp);
                    self.pos += 2;
                }
            },
            // output
            4 => {
                let out = self.read_mode(instr, 0);
                if self.log {
                    println!("{}", out);
                }
                self.outputs.push_back(out);
                self.pos += 2;
            },
            // jump if true
            5 => self.cond_jump_un(instr, |x| x!=0),
            // jump if false
            6 => self.cond_jump_un(instr, |x| x==0),
            // less than
            7 => self.binop(instr, |x,y| if x<y { 1 } else { 0 }),
            // equals
            8 => self.binop(instr, |x,y| if x==y { 1 } else { 0 }),
            // adjust relative base
            9 => {
              let arg1 = self.read_mode(instr, 0);
              self.rb += arg1;
              self.pos += 2
            }
            _ => panic!("Invalid instruction")
        }
        self.state.clone()
    }
}
