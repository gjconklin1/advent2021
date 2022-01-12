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

  let mut forward = 0;
  let mut depth = 0;

  for (_index, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    let mut split = line.split_whitespace();

    let command = split.next().unwrap();
    let distance: u32 = split.next().unwrap().parse().unwrap();

    match command {
      "forward" => {forward = forward + distance;},
      "down" => {depth = depth + distance;},
      "up" => {depth = depth - distance;},
      _ => ()
    }
  }
  println!("{} x {} = {}", forward, depth, forward * depth)
}

pub fn run_part2(input: &PathBuf) {
  let file = File::open(input).expect("Unable to open file.");
  let reader = BufReader::new(file);

  let mut aim = 0;
  let mut forward = 0;
  let mut depth = 0;

  for (_index, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    let mut split = line.split_whitespace();

    let command = split.next().unwrap();
    let amount: u32 = split.next().unwrap().parse().unwrap();

    match command {
      "forward" => {
        forward = forward + amount;
        depth = depth + (aim * amount)
      },
      "down" => {aim = aim + amount;},
      "up" => {aim = aim - amount;},
      _ => ()
    }
  }
  println!("{} x {} = {}", forward, depth, forward * depth)
}
