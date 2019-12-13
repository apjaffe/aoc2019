extern crate aoc;

use std::io::{self,Bytes,Read,StdinLock};
use std::fs;
use std::collections::HashMap;
use aoc::Computer;

struct Screen {
  pixels: HashMap<(i64,i64),i64>,
  text: i64,
  dir: i64
}

impl Screen {
  fn new() -> Screen {
    Screen {
      pixels: HashMap::new(),
      text: 0,
      dir: 0
    }
  }
  fn get_xcoords(self: &Screen, pixel:i64) -> i64 {
    *self.pixels.iter().filter(|((x,y),v)| **v==pixel).map(|((x,y),v)| x).min().unwrap()
  }
  fn read_frame(self: &mut Screen, comp: &mut Computer, term: &mut Bytes<StdinLock>) -> bool {
    if let (Some(x), Some(y), Some(tile)) =
      (comp.run_until_output(), comp.run_until_output(), comp.run_until_output()) {
        if x == -1 && y == 0 {
          self.text = tile;
        } else {
          self.pixels.insert((x,y), tile);
        }
      return true;
    }
    let ballx = self.get_xcoords(4);
    let paddlex = self.get_xcoords(3);
    println!("{} {}", ballx, paddlex);
    if ballx < paddlex {
      comp.input(-1);
    } else if ballx > paddlex {
      comp.input(1);
    } else {
      comp.input(0);
    }
    println!("");
    /*let ch = term.next().unwrap().unwrap();
    match ch {
      97 => { self.dir = -1; term.next(); },
      100 => { self.dir = 1; term.next(); },
      115 => { self.dir = 0; term.next(); },
      _ => {  }
    }
    comp.input(self.dir);*/
    false
  }
  fn render(self: &Screen) {
    println!("Score: {}", self.text);
    let xcoords = self.pixels.keys().map(|(x,_y)|x).collect::<Vec<&i64>>();
    let ycoords = self.pixels.keys().map(|(_x,y)|y).collect::<Vec<&i64>>();
    let minx = **xcoords.iter().min().unwrap();
    let maxx = **xcoords.iter().max().unwrap();
    let miny = **ycoords.iter().min().unwrap();
    let maxy = **ycoords.iter().max().unwrap();
    for y in miny..(maxy+1) {
        for x in minx..(maxx+1) {
            if let Some(x) = self.pixels.get(&(x,y)) {
                print!("{}", x);
            } else {
                print!(" ");
            }
        }
        println!("");
    }
  }
}

fn main() {
    let mut buffer = fs::read_to_string("inputs/day13").unwrap();
    let mut comp = Computer::new_adv(&buffer, false);
    let mut screen = Screen::new();
    comp.write(0, 2);

    let mut stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut term = reader.bytes();

    while !comp.is_halted() {
      while screen.read_frame(&mut comp, &mut term) {
        screen.render();
      }
    }
}
