use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
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
  for line in reader.lines() {
    let line = line.unwrap();
    let mut split = line.split(" | ");
    let outputs = split.next().unwrap();

    let outputs: Vec<&str> = outputs.split_whitespace().collect();
    for output in outputs {
      count += match output.chars().count() {
        2 => 1,
        4 => 1,
        3 => 1,
        7 => 1,
        _ => 0
      }
    }
  }
  println!("count = {}", count);
}


// Notes...

// len 2 -> "1"
// len 3 -> "7"
// len 4 -> "4"
// len 7 -> "8"
// len 5 -> "2", "3", "5"
// len 6 -> "0", "6", "9"


fn determine_digit(o: &str, c_or_f: &HashSet<char>, b_or_d: &HashSet<char>) -> i32 {
  if o.len() == 2 { return 1; }
  if o.len() == 3 { return 7; }
  if o.len() == 4 { return 4; }
  if o.len() == 7 { return 8; }

  let mut c_or_f_count = 0;
  let mut b_or_d_count = 0;
  for char in o.chars() {
    if c_or_f.contains(&char) {c_or_f_count += 1;}
    if b_or_d.contains(&char) {b_or_d_count += 1;}
  }

  if o.len() == 5 {
    if c_or_f_count == 2 { return 3;}
    if b_or_d_count == 2 { return 5;}
    return 2;
  }

  if c_or_f_count == 1 { return 6;}
  if b_or_d_count == 2 { return 9;}
  return 0;
}


pub fn run_part2(input: &PathBuf) {
  let file = File::open(input).expect("Unable to open file.");
  let reader = BufReader::new(file);

  let mut sum = 0;
  for line in reader.lines() {
    // Load an parse
    let line = line.unwrap();
    let mut split = line.split(" | ");
    let inputs = split.next().unwrap();
    let outputs = split.next().unwrap();

    let inputs: Vec<&str> = inputs.split_whitespace().collect();
    let outputs: Vec<&str> = outputs.split_whitespace().collect();

    let mut num1_options: HashSet<char> = HashSet::new();
    let mut num7_options: HashSet<char> = HashSet::new();
    let mut num4_options: HashSet<char> = HashSet::new();

    // First pass: fill all options
    for input in inputs {
      match input.chars().count() {
        2 => num1_options = input.chars().collect(),
        3 => num7_options = input.chars().collect(),
        4 => num4_options = input.chars().collect(),
        _ => ()
      }
    }

    // Now create two HashSets to contain the signals for c_or_f
    let mut c_or_f: HashSet<char> = HashSet::new();
    if num1_options.len() > 0 {
      for c in &num1_options { c_or_f.insert(*c); }
    }
    else if num4_options.len() > 0 && num7_options.len() > 0 {
      for c in &num4_options {
        if num7_options.contains(&c) { c_or_f.insert(*c); }
      }
    }
    else { panic!("Could not make c_or_f!"); }

    // Now create two HashSets to contain the signals for b_or_d
    let mut b_or_d: HashSet<char> = HashSet::new();
    if num4_options.len() > 0 && num1_options.len() > 0 {
      for c in &num4_options {
        if !num1_options.contains(&c) { b_or_d.insert(*c); }
      }
    }
    else if num4_options.len() > 0 && num7_options.len() > 0{
      for c in &num4_options {
        if !num7_options.contains(&c) { b_or_d.insert(*c); }
      }
    }
    else { panic!("Could not make b_or_d!"); }

    let mut value = 0;
    for output in outputs {
      value *= 10;
      value += determine_digit(output, &c_or_f, &b_or_d);
    }
    println!("value = {}", value);
    sum += value;
  }

  println!("sum = {}", sum)
}