use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;


pub fn run(input: Option<PathBuf>) {

  let file = File::open(input.unwrap()).expect("Unable to open file.");
  let reader = BufReader::new(file);
  
  let mut map: Vec<Vec<i32>> = Vec::new();

  // Build map
  for line in reader.lines() {
    let mut row: Vec<i32> = Vec::new();
    for c in line.unwrap().chars() {
      let energey: i32 = c.to_digit(10).unwrap().try_into().unwrap();
      row.push(energey);
    }
    map.push(row);
  }
  let height = map.len();
  let width = map[0].len();

  let mut total_flash_count = 0;
  let mut first_all_flash = 0;

  for step in 1..=2000 {
    // inc and spread
    for y in 0..height {
      for x in 0..width {
        inc_and_spread(&mut map, x, y);
      }
    }

    // reset
    let mut num_flashing = 0;
    for y in 0..height {
      for x in 0..width {
        if map[y][x] == -1 { 
          map[y][x] = 0; 
          total_flash_count += 1;
          num_flashing += 1;
        }
      }
    }
    
    if step % 50 == 0 {
      println!();
      println!("-- Step {} --", step);
      print_map(&map);  
    }

    if first_all_flash == 0 && num_flashing == width * height {
      first_all_flash = step;
      break;
    }

  }

  println!("Flash count: {}", total_flash_count);
  println!("First flash step: {}", first_all_flash);
}



fn inc_and_spread(map: &mut Vec<Vec<i32>>, x: usize, y: usize) {

  if map[y][x] != -1 {
    map[y][x] += 1;
  }

  if map[y][x] > 9 {

    // Flash!
    map[y][x] = -1;

    // Spread if flash...

    if y > 0 {
      inc_and_spread(map, x, y - 1);
      if x > 0 {
        inc_and_spread(map, x - 1, y - 1);
      }
      if x < map[0].len() - 1 {
        inc_and_spread(map, x + 1, y - 1);
      }
    }
    if y < map.len() - 1 {
      inc_and_spread(map, x, y + 1);
      if x > 0 {
        inc_and_spread(map, x - 1, y + 1);
      }
      if x < map[0].len() - 1 {
        inc_and_spread(map, x + 1, y + 1);
      }
    }
    if x > 0 { 
      inc_and_spread(map, x - 1, y); 
    }
    if x < map[0].len() - 1 {
      inc_and_spread(map, x + 1, y);
    }
  }
}


fn print_map(map: &Vec<Vec<i32>>) {
  let height = map.len();
  let width = map[0].len();

  for y in 0..height {
    for x in 0..width {
      print!("{}", map[y][x]);
    }
    println!();
  }
}