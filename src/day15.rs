use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::collections::HashSet;


// Dijkstra FTW

pub fn run(input: Option<PathBuf>) {

  // Open the file and get a reader
  let file = File::open(input.unwrap()).expect("Unable to open file.");
  let reader = BufReader::new(file);

  let mut sub_risk_map: Vec<Vec<u32>> = Vec::new();
  let mut dist_map: Vec<Vec<u32>> = Vec::new();
  let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
  let mut tentatives: HashSet<(usize, usize)> = HashSet::new();

  // Build risk map
  for line in reader.lines() {
    let mut row: Vec<u32> = Vec::new();
    for c in line.unwrap().chars() {
      let risk: u32 = c.to_digit(10).unwrap();
      row.push(risk);
    }
    sub_risk_map.push(row);
  }

  let sub_width = sub_risk_map[0].len();
  let sub_height = sub_risk_map.len();

  // In part 2 the map is 5x5x bigger...
  let width = 5 * sub_width;
  let height = 5 * sub_height;
  let mut risk_map: Vec<Vec<u32>> = Vec::new();
  for y in 0..height {
    let mut row: Vec<u32> = Vec::new();
    for x in 0..width {
      row.push(calc_risk(&sub_risk_map, (x, y)));
    }
    risk_map.push(row);
  }

  // Initialize distance map
  for _y in 0..height {
    let mut row: Vec<u32> = Vec::new();
    for _x in 0..width {
      row.push(u32::MAX);
    }
    dist_map.push(row);
  }

  // Initialize unvisited Set
  for x in 0..width {
    for y in 0..height {
      unvisited.insert((x, y));
    }
  }
  
  // Initialize current and destination nodes
  let mut current_node: (usize, usize) = (0, 0);
  dist_map[0][0] = 0;
  let destination_node: (usize, usize) = (width - 1, height - 1);

  for i in 0..1000000 {
    let cx = current_node.0;
    let cy = current_node.1;
    let current_distance = dist_map[cy][cx];

    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if cx < width - 1 { neighbors.push((cx + 1, cy)); }
    if cy < height - 1 { neighbors.push((cx, cy + 1)); }
    if cx > 0 { neighbors.push((cx - 1, cy)); }
    if cy > 0 { neighbors.push((cx, cy - 1)); }

    for neighbor in neighbors {
      if unvisited.contains(&neighbor) { 
        let nx = neighbor.0;
        let ny = neighbor.1;

        let tentative_distance = current_distance + risk_map[ny][nx];

        if dist_map[ny][nx] > tentative_distance {
          dist_map[ny][nx] = tentative_distance;
          tentatives.insert(neighbor);
        }
      }
    }

    unvisited.remove(&current_node);
    tentatives.remove(&current_node);

    if current_node.eq(&destination_node) {
      println!();
      println!("Arrived with a distance of {}.", current_distance);
      break;
    }

    let mut smallest_unvisited_distance = u32::MAX;
    for node in &tentatives {
      let ux = node.0;
      let uy = node.1;
      if dist_map[uy][ux] < smallest_unvisited_distance {
        smallest_unvisited_distance = dist_map[uy][ux];
        current_node = *node;
      }
    }

    print!("Step {} ---- \r", i + 1);
  }
}


fn calc_risk(risk_map: &Vec<Vec<u32>>, point: (usize, usize)) -> u32 {
  let x = point.0;
  let y = point.1;
  let map_width = risk_map[0].len();
  let map_height = risk_map.len();

  let kicker: u32 = ((y / map_height) + (x / map_width)) as u32;

  let risk = risk_map[y % map_height][x % map_width] + kicker;
  let risk = (risk % 10) + (risk / 10);

  risk
}


fn _print_risk_map(map: &Vec<Vec<u32>>, width: usize, height: usize) {
  for y in 0..height {
    for x in 0..width {
      print!("{}", calc_risk(&map, (x, y)));
      if x % 10 == 9 { print!(" "); }
    }
    println!();
    if y % 10 == 9 { println!(""); }
  }
}

