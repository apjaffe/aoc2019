extern crate aoc;

use std::io::{self,Read};
use std::collections::{HashMap,VecDeque,HashSet};
use aoc::Computer;

#[derive(Eq,PartialEq,Hash,Clone,Copy,Debug)]
struct Coord {
  x: i64,
  y: i64
}

struct Move {
  coord: Coord,
  dist: i64,
  dir: i64
}

fn opposite(dir: i64) -> i64 {
  match dir {
    1 => 2,
    2 => 1,
    3 => 4,
    4 => 3,
    _ => panic!("bad dir {}", dir)
  }
}

fn parse_dir(dir: i64) -> Coord {
  match dir {
    1 => Coord{x: 0, y: 1},
    2 => Coord{x: 0, y: -1},
    3 => Coord{x: -1, y: 0},
    4 => Coord{x: 1, y: 0},
    _ => panic!("bad dir {}", dir)
  }
}

fn find_dir(start: &Coord, end: &Coord) -> i64 {
  match (end.x-start.x, end.y-start.y) {
    (0,1) => 1,
    (0,-1) => 2,
    (-1,0) => 3,
    (1,0) => 4,
    _ => panic!("bad offset")
  }
}

fn add_dir(start: &Coord, dir: i64) -> Coord {
  let add = parse_dir(dir);
  Coord {
    x: start.x + add.x,
    y: start.y + add.y
  }
}

fn find_path(start: Coord, end: Coord, prevs: HashMap<Coord, Coord>) -> Vec<i64>{
  let mut route = Vec::new();
  let mut next = end;
  loop {
    match prevs.get(&next) {
      Some(prev) => {
        if next == start {
          route.reverse();
          return route;
        }
        route.push(find_dir(&prev, &next));
        next = *prev;
      },
      None => {
        if next != start {
          panic!("Bad route: {:?}", next)
        }
        route.reverse();
        return route;
      }
    }
  }
}

fn bfs_offline(start: Coord, end: Coord, map: &HashMap<Coord, i64>) -> Vec<i64> {
  let mut visited = HashSet::new();
  let mut prevs = HashMap::new();
  let mut queue = VecDeque::new();
  queue.push_back((start, None));
  while let Some((coord, prev)) = queue.pop_front() {
    if visited.contains(&coord) {
      continue;
    }
    visited.insert(coord);
    if coord == end {
      return find_path(start, end, prevs);
    }
    for dir in 1..5 {
      let next = add_dir(&coord, dir);
      let movable = match map.get(&next) {
        None => false,
        Some(0) => false,
        Some(1) => true,
        Some(2) => true,
        _ => panic!("invalid")
      };
      if movable {
        queue.push_back((next, Some(coord)));
        if !prevs.contains_key(&next) {
          prevs.insert(next, coord);
        }
      }
    }
  }
  panic!("unreachable");
}

fn dfs_online(comp: &mut Computer, map: &mut HashMap<Coord, i64>, pos: &Coord, cur: i64) {
  if map.contains_key(pos) {
    return;
  }
  map.insert(*pos, cur);
  for dir in 1..5 {
    comp.input(dir);
    let next_pos = add_dir(pos, dir);
    let res = comp.run_until_output().unwrap();
    if res == 1 || res == 2 {
      dfs_online(comp, map, &next_pos, res);
      comp.input(opposite(dir));
      let res = comp.run_until_output().unwrap();
      if res != cur {
        panic!("map broken");
      }
    }
  }
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();
    let mut comp = Computer::new_adv(&buffer, false);
    let mut map = HashMap::new();
    let mut start = Coord {x:0, y:0};
    dfs_online(&mut comp, &mut map, &start, 1);
    //println!("{:?}", map);
    let target = map.iter().filter(|(coord, typ)| **typ == 2).collect::<Vec<(&Coord, &i64)>>();
    if target.len() != 1 {
      panic!("wrong num targets");
    }
    println!("{:?}",target);
    let ox = *target[0].0;
    let path = bfs_offline(start, ox, &map);
    println!("{:?}", path);
    println!("{:?}", path.len());
    // okay so yes we should really just do one BFS and see the max distance instead of repeating
    // the BFS for every target pos but this way is just a lot faster to write...
    let ox_time = map.keys().map(|pos| bfs_offline(ox, *pos, &map).len()).max().unwrap();
    println!("{:?}", ox_time);
}
