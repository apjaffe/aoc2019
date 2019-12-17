use std::io::{self, BufRead};

fn fft(in_list: &Vec<i64>) -> Vec<i64> {
    let pattern = vec![0, 1, 0, -1];
    let pattern_len = pattern.len();
    let mut out = Vec::new();
    for out_idx in 0..in_list.len() {
      let mut sum = 0;
      /*for (in_idx, in_val) in in_list.iter().enumerate() {
        let pattern_pos = ((in_idx+1) / (out_idx+1)) % pattern_len;
        sum += in_val * pattern[pattern_pos]
      }*/
      /*let mut in_idx = out_idx;
      while in_idx < in_list.len() {
        sum += in_val[in_idx..
      }*/
      for (in_idx, in_val) in in_list.iter().enumerate() {
        let pattern_pos = ((in_idx+1) / (out_idx+1)) % pattern_len;
        sum += in_val * pattern[pattern_pos]
      }
      out.push(sum.abs() % 10);
    }
    out
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut vec = line.chars().map(|x| x.to_digit(10).unwrap() as i64).collect::<Vec<i64>>();
    //println!("{:?}", vec);
    for _i in 0..100 {
      vec = fft(&vec);
    }
    //println!("{:?}", vec);
    println!("{:?}", &vec[0..8]);
}
