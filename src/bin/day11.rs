extern crate aoc;

use std::io::{self,Read};
use std::collections::HashMap;
use aoc::Computer;

fn cos_sin(dir:i64) -> (i64, i64) {
    let dir = dir % 360;
    let dir = if dir < 0 {
        dir + 360
    } else {
        dir
    };
    match dir % 360 {
        0 => (1, 0),
        90 => (0, 1),
        180 => (-1, 0),
        270 => (0, -1),
        _ => panic!("Bad angle: {}", dir)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();
    let mut comp = Computer::new_adv(&buffer, false);

    let mut paint = HashMap::new();
    paint.insert((0,0),1);
    let mut location = (0,0);
    let mut dir = 270;

    while !comp.is_halted() {
        comp.input(*paint.get(&location).unwrap_or(&0));
        comp.run_until_waiting();
        if let Some(paint_out) = comp.output() {
            paint.insert(location, paint_out);
            if let Some(dir_out) = comp.output() {
                dir += match dir_out {
                    0 => -90,
                    1 => 90,
                    _ => panic!("Bad turn")
                };
                let (dx, dy) = cos_sin(dir);
                location = (location.0 + dx, location.1+dy);
            }
        }
    }
    println!("{}", paint.len());
    let xcoords = paint.keys().map(|(x,_y)|x).collect::<Vec<&i64>>();
    let ycoords = paint.keys().map(|(_x,y)|y).collect::<Vec<&i64>>();
    let minx = **xcoords.iter().min().unwrap();
    let maxx = **xcoords.iter().max().unwrap();
    let miny = **ycoords.iter().min().unwrap();
    let maxy = **ycoords.iter().max().unwrap();
    for y in miny..(maxy+1) {
        for x in minx..(maxx+1) {
            if let Some(1) = paint.get(&(x,y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
