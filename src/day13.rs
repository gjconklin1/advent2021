use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::path::PathBuf;


#[derive(Debug)]
enum Fold {
  AlongX(u32),
  AlongY(u32),
}

pub fn run(input: Option<PathBuf>) {
  // Open the file and get a reader
  let file = File::open(input.unwrap()).expect("Unable to open file.");
  let reader = BufReader::new(file);

  let mut dots: HashSet<(u32, u32)> = HashSet::new();
  let mut folds: Vec<Fold> = Vec::new();

  // Parse file into `dots` and `folds`
  for line in reader.lines() {
    let line = line.expect("Cannot read line.");
    if line.starts_with("fold along") {
      let line_split: Vec<&str> = line.split("=").collect();
      let fold_point: u32 = line_split[1].parse().unwrap();
      let fold = match line.chars().nth(11).unwrap() {
        'x' => Some(Fold::AlongX(fold_point)),
        'y' => Some(Fold::AlongY(fold_point)),
        _ => None
      };
      folds.push(fold.unwrap());
    }
    else {
      let line_split: Vec<&str> = line.split(",").collect();
      if line_split.len() == 2 {
        let x: u32 = line_split[0].parse().unwrap();
        let y: u32 = line_split[1].parse().unwrap();
        dots.insert((x, y));
      }
    }
  }


  // Perform folds
  let mut fold_count = 0;
  println!("-- fold {}: dot count -> {} ", fold_count, dots.len());

  for fold in folds {
    let mut dots_next: HashSet<(u32, u32)> = HashSet::new();

    for dot in dots {

      match fold {
        Fold::AlongX(fold_x) => {
          match dot.0 > fold_x {
            true => {
              let new_x = fold_x - (dot.0 - fold_x);
              dots_next.insert((new_x, dot.1));
            },
            false => {
              dots_next.insert(dot);
            },
          }
        },
        Fold::AlongY(fold_y) => {
          match dot.1 > fold_y {
            true => {
              let new_y = fold_y - (dot.1 - fold_y);
              dots_next.insert((dot.0, new_y));
            }
            false => {
              dots_next.insert(dot);
            },
          }
        },
      }   
    }

    dots = dots_next;
    fold_count += 1;

    println!("-- fold {}: dot count -> {} ", fold_count, dots.len());
  }

  println!();
  println!("The code is...");
  println!();
  
  for y in 0..6 {
    for x in 0..40 {
      match dots.get(&(x, y)) {
        Some(_dot) => print!("██"),
        None       => print!("  "),
      }
    }
    println!();
  }
  println!();

}
