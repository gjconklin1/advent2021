use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::path::PathBuf;


pub fn run(input: Option<PathBuf>) {
  let file = File::open(input.unwrap()).expect("Unable to open file.");
  let reader = BufReader::new(file);
  
  let mut map: Vec<Vec<u32>> = Vec::new();

  // Build map
  for line in reader.lines() {
    let mut row: Vec<u32> = Vec::new();
    for c in line.unwrap().chars() {
      let depth: u32 = c.to_digit(10).unwrap();
      row.push(depth);
    }
    map.push(row);
  }

  println!("Part 1:");

  // Find low points
  let height = map.len();
  let width = map[0].len();

  let mut low_points: HashSet<(usize, usize)> = HashSet::new();
  let mut count = 0;
  let mut risk_level = 0;

  for y in 0..height {
    for x in 0..width {
      let mut is_low_point = true;
      if x > 0 {
        if map[y][x-1] <= map[y][x] { is_low_point = false; }
      }
      if x < width - 1 {
        if map[y][x+1] <= map[y][x] { is_low_point = false; }
      }
      if y > 0 {
        if map[y-1][x] <= map[y][x] { is_low_point = false; }
      }
      if y < height - 1 {
        if map[y+1][x] <= map[y][x] { is_low_point = false; }
      }

      if is_low_point { 
        count += 1;
        risk_level += map[y][x] + 1;
        low_points.insert((x, y));
      }
    }
  }

  println!(" count = {}", count);
  println!(" risk_level = {}", risk_level);

  // Part 2
  // Basin is represented as a HashSet of (x, y) Tuples.
  // Collected in a Vector.

  println!("Part 2:");

  let mut basin_sizes: Vec<usize> = Vec::new();
  for low_point in &low_points {
    let basin = find_basin(&map, low_point.0, low_point.1);
    basin_sizes.push(basin.len());
  }
  basin_sizes.sort_by(|a, b| b.cmp(a));
  println!(" Basin Top Sizes: {} x {} x {} = {}", 
    basin_sizes[0], basin_sizes[1], basin_sizes[2],
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}


fn find_basin(map: &Vec<Vec<u32>>, x: usize, y: usize) -> HashSet<(usize, usize)> {
  let mut basin: HashSet<(usize, usize)> = HashSet::new();

  let mut to_test: HashSet<(usize, usize)> = HashSet::new();
  let mut tested: HashSet<(usize, usize)> = HashSet::new();

  let height = map.len();
  let width = map[0].len();

  to_test.insert((x, y));

  while to_test.len() > 0 {
    let drain: &HashSet<(usize, usize)> = &to_test.drain().collect();
    for test in drain {
      let x = test.0;
      let y = test.1;

      tested.insert(*test);

      if map[test.1][test.0] < 9 {
        basin.insert(*test);

        // Above
        if y > 0 {
          let above = (x, y - 1);
          if !tested.contains(&above) { to_test.insert(above); }
        }

        // Below
        if y < height - 1 {
          let below = (x, y + 1);
          if !tested.contains(&below) { to_test.insert(below); }
        }

        // Left
        if x > 0 {
          let left = (x - 1, y);
          if !tested.contains(&left) { to_test.insert(left); }
        }

        // Right
        if x < width - 1 {
          let right = (x + 1, y);
          if !tested.contains(&right) { to_test.insert(right); }
        }
      }
    }
  }

  basin
}


/*
9234598321279999876543212397634598789843210123456789212999878987556456999878965432459101987654567899
8965987432367898989864301986545789698765421939697894349876967896432349899769896543598929898763478998
*/