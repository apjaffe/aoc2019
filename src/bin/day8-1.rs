use std::io::{self,Read};

fn count_digits(layer: Vec<Vec<u8>>) -> Vec<i32> {
  let mut counts = Vec::new();
  counts.resize(10, 0);
  for digit in 0..10 {
    counts[digit] = layer.iter().flatten().map(|x| if *x as usize == digit { 1_i32 } else { 0_i32 }).sum();
  }
  return counts;
}

fn load_img(input:&str) -> Vec<Vec<Vec<u8>>> {
  let mut img: Vec<Vec<Vec<u8>>> = Vec::new();
  let mut bytes = input.bytes();
  loop {
    let mut layer = Vec::new();
    for _y in 0..6 {
      let mut row = Vec::new();
      for _x in 0..25 {
        if let Some(byte) = bytes.next() {
          if byte < 48_u8 {
            return img;
          }
          row.push(byte-48_u8);
        } else {
          return img;
        }
      }
      layer.push(row);
    }
    img.push(layer);
  }
}
fn main() {
  let stdin = io::stdin();
  let mut buffer = String::new();
  let mut handle = stdin.lock();
  handle.read_to_string(&mut buffer).unwrap();
  let img = load_img(&buffer);
  let mut best_k = 0;
  let mut best_v = 25*6;
  for layer in img {
    let counts = count_digits(layer);
    if counts[0] < best_v {
      best_v = counts[0];
      best_k = counts[1] * counts[2];
    }
  }
  println!("{}", best_k);
}
