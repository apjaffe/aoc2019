use std::io::{self,Read};

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

fn top_pixels(img: Vec<Vec<Vec<u8>>>) -> Vec<Vec<u8>> {
  let mut out: Vec<Vec<u8>> = Vec::new();
  for _y in 0..6 {
    let mut row = Vec::new();
    row.resize(25,2);
    out.push(row);
  }
  for layer in img {
    for (y, row) in layer.iter().enumerate() {
      for (x, pixel) in row.iter().enumerate() {
        if out[y][x] == 2 {
          out[y][x] = *pixel;
        }
      }
    }
  }
  out
}

fn main() {
  let stdin = io::stdin();
  let mut buffer = String::new();
  let mut handle = stdin.lock();
  handle.read_to_string(&mut buffer).unwrap();
  let img = load_img(&buffer);
  let render = top_pixels(img);
  for row in render {
    println!("{:?}", row.iter().map(|x| if *x == 0_u8 {' '} else {'#'} ).collect::<Vec<char>>());
  }
}
