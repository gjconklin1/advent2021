use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::path::PathBuf;


pub fn run(input: Option<PathBuf>) {

  // (x, y) -> num_vents
  let mut vents: HashMap<(i32, i32), u32> = HashMap::new();

  // Open the file and get a reader
  let file = File::open(input.unwrap()).expect("Unable to open file.");
  let reader = BufReader::new(file);
  
  for line in reader.lines() {
    let line = line.expect("unable to read line.");
    let endpoints: Vec<&str> = line.split(" -> ").collect();
    let coord1: Vec<i32> = endpoints[0].split(",").map(|s| s.parse().unwrap()).collect();
    let coord2: Vec<i32> = endpoints[1].split(",").map(|s| s.parse().unwrap()).collect();

    // Vertical
    if coord1[0] == coord2[0] {
      let range = match coord2[1] > coord1[1] {
        true => coord1[1]..=coord2[1],
        false => coord2[1]..=coord1[1],
      };
      for y in range {
        *vents.entry((coord1[0], y)).or_insert(0) += 1;
      }
    }

    // Horizontal
    else if coord1[1] == coord2[1] {
      let range = match coord2[0] > coord1[0] {
        true => coord1[0]..=coord2[0],
        false => coord2[0]..=coord1[0],
      };
      for x in range {
        *vents.entry((x, coord1[1])).or_insert(0) += 1;
      }
    }

    // Diagonal
    else {
      let mut x = coord1[0];
      let dx = match coord1[0] < coord2[0] {true => 1, false => -1};
      let mut y = coord1[1];
      let dy = match coord1[1] < coord2[1] {true => 1, false => -1};
      loop {
        *vents.entry((x, y)).or_insert(0) += 1;
        if x == coord2[0] {break;}
        x += dx;
        y += dy;
      }
    }
  }

  /*
  for y in 0..10 {
    for x in 0..10 {
      match vents.get(&(x,y)) {
        None => {print!(".")},
        Some(i) => {print!("{}", i)}
      };
    }
    println!()
  }
  */
  
  let mut count = 0;
  for key in vents.keys() {
    if *vents.get(key).unwrap() > 1 {count += 1;}
  }  

  println!("count = {}", count);

}