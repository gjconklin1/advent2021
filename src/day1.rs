use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;


pub fn run(input: Option<PathBuf>) {
  let input = input.unwrap();
  run_part1(&input);
  run_part2(&input);
}

pub fn run_part1(input: &PathBuf) {
  let file = File::open(input).expect("Unable to open file.");
  let reader = BufReader::new(file);

  let mut count = 0;
  let mut prev_depth = 0;
  for (_index, line) in reader.lines().enumerate() {
    let depth: u32 = line.unwrap().parse().unwrap();
    if prev_depth != 0 && depth > prev_depth {count = count + 1;}
    prev_depth = depth;
  }
  println!("Count of increases: {}", count)
}

pub fn run_part2(input: &PathBuf) {
  let file = File::open(input).expect("Unable to open file.");
  let reader = BufReader::new(file);

  let mut count = 0;
  let mut window = Vec::new();
  let mut sum = 0;
  let mut prev_sum = 10000;
  for (_index, line) in reader.lines().enumerate() {
    let depth: u32 = line.unwrap().parse().unwrap();
    sum = sum + depth;
    window.push(depth);
    if window.len() == 4 {
      let removed = window.remove(0);
      sum = sum - removed;
    }
    if window.len() == 3 {
      if sum > prev_sum {count = count + 1;}
      prev_sum = sum;
    }
  }
  println!("Count of moving window increases: {}", count)
}
