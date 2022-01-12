use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::collections::HashMap;


pub fn run(input: Option<PathBuf>) {

  let mut pair_counts: HashMap<(char, char), u64> = HashMap::new();
  let mut insertions: Vec<((char, char), char)> = Vec::new();
  let mut char_counts: HashMap<char, u64> = HashMap::new();

  // Open the file and get a reader
  let file = File::open(input.unwrap()).expect("Unable to open file.");
  let mut reader = BufReader::new(file);

  // Read first line
  let mut first_line = String::new();
  reader.read_line(&mut first_line).unwrap();
  let mut prev_c = ' ';
  for c in first_line.chars() {
    if c != '\n' {
      *char_counts.entry(c).or_insert(0) += 1;
      if prev_c != ' ' {
        *pair_counts.entry((prev_c, c)).or_insert(0) += 1;
      }
    }
    prev_c = c;
  }

  // Read insertion list
  for line in reader.lines() {
    let line = line.unwrap();
    let split: Vec<&str> = line.split(" -> ").collect();
    if split.len() == 2 {
      let first = split[0].chars().nth(0).unwrap();
      let second = split[0].chars().nth(1).unwrap();
      let insert = split[1].chars().nth(0).unwrap();
      insertions.push(((first, second), insert));
    }
  }

  println!();
  println!("-------After step 0: ");
  println!("char_counts = {:?}", char_counts);
  println!("Polymer size = {:?}", char_counts.values().sum::<u64>());

  // Do insertions
  for step in 0..40 {
    
    let mut added_pairs: HashMap<(char, char), u64> = HashMap::new();
    let mut removed_pairs: Vec<(char, char)> = Vec::new();

    for insertion in &insertions {
      
      match pair_counts.get(&insertion.0) {
        Some(count) => {
          *added_pairs.entry((insertion.0.0, insertion.1)).or_insert(0) += count;
          *added_pairs.entry((insertion.1, insertion.0.1)).or_insert(0) += count;
          removed_pairs.push(insertion.0);

          *char_counts.entry(insertion.1).or_insert(0) += count;
        },
        None => ()
      }

    } 
    for k in &removed_pairs {
      pair_counts.remove(k);
    }
    for (k,v) in &added_pairs {
      *pair_counts.entry(*k).or_insert(0) += v;
    }

    let max_count = char_counts.values().max().unwrap();
    let min_count = char_counts.values().min().unwrap();

    println!();
    println!("-------After step {}: ", step +  1);
    println!("char_counts = {:?}", char_counts);
    println!("Polymer size = {:?}", char_counts.values().sum::<u64>());
    println!(" {} - {} = {}", max_count, min_count, max_count - min_count);

  }


}

