use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::PathBuf;


pub fn run(input: Option<PathBuf>) {
  let input = input.unwrap();
  run_part1(&input);
  run_part2(&input);
}

pub fn run_part1(input: &PathBuf) {
  // Open the file and get a reader
  let file = File::open(input).expect("Unable to open file.");
  let mut reader = BufReader::new(file);

  // This vector holds the bit counts by position
  let mut bit_counts: Vec<i32> = Vec::new();

  // Pull out the first line to start the counting
  let mut first_line = String::new();
  reader.read_line(&mut first_line).unwrap();

  let bits = first_line.as_bytes();
  for (_index, &bit) in bits.iter().enumerate() {
    match bit {
      b'0' => bit_counts.push(-1),
      b'1' => bit_counts.push(1),
      _ => ()
    }
  }

  // Process the rest of the lines
  for (_index, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    let bits = line.as_bytes();
    for (index, &bit) in bits.iter().enumerate() {
      match bit {
        b'0' => bit_counts[index] -= 1,
        b'1' => bit_counts[index] += 1,
        _ => ()
      }
    }
  }

  // Form gamma rate and epsilon rate
  let mut gamma_rate = 0;
  let mut epsilon_rate = 0;
  for c in bit_counts {
    gamma_rate *= 2;
    epsilon_rate *= 2;
    match c > 0 {
      true => gamma_rate += 1,
      false => epsilon_rate += 1,
    }
  }
  println!("gamma = {}, epsilon = {}, g x e = {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}



fn find_bit_balance(lines: &Vec<String>, bit_position: usize) -> i32 {
  // Figure out whether '1' or '0' is most common in the
  // given bit_position
  let mut bit_balance = 0;
  for line in lines {
    let bit: u8 = line.as_bytes()[bit_position];
    match bit {
      b'0' => bit_balance -= 1,
      b'1' => bit_balance += 1,
      _ => ()
    }
  }

  return bit_balance;
}


fn filter_by(lines: &Vec<String>, bit_position: usize, filter_bit: u8) -> Vec<String> {

  // Scan and pull out the matching readings
  let mut filtered_readings: Vec<String> = Vec::new(); 
  for line in lines {
    let bit: u8 = line.as_bytes()[bit_position];
    if bit.eq(&filter_bit) {
      filtered_readings.push(line.to_string());
    }
  }

  filtered_readings
}


fn filter_for_o2(lines: &Vec<String>, bit_position: usize) -> Vec<String> {
  let bit_balance = find_bit_balance(&lines, bit_position);
  let filter_bit = match bit_balance >= 0 {
    true => b'1',
    false => b'0'
  };
  filter_by(lines, bit_position, filter_bit)
}


fn filter_for_co2(lines: &Vec<String>, bit_position: usize) -> Vec<String> {
  let bit_balance = find_bit_balance(&lines, bit_position);
  let filter_bit = match bit_balance >= 0 {
    false => b'1',
    true => b'0'
  };
  filter_by(lines, bit_position, filter_bit)
}


pub fn run_part2(input: &PathBuf) {

  // Open the file and get a reader
  let file = File::open(input).expect("Unable to open file.");
  let reader = BufReader::new(file);
  
  let lines = reader.lines().collect::<Result<Vec<String>>>().unwrap();
  
  // O2 filtering
  let mut bit_position = 0;
  let mut filtered_o2_readings = filter_for_o2(&lines, bit_position);
  while filtered_o2_readings.len() > 1 {
    bit_position += 1;
    filtered_o2_readings = filter_for_o2(&filtered_o2_readings, bit_position);
  }

  let o2_reading = &filtered_o2_readings[0];
  let o2_value = isize::from_str_radix(&o2_reading, 2).unwrap();
  println!("Oxygen Filtering: {} -> {}", &o2_reading, &o2_value);

  // CO2 filtering
  let mut bit_position = 0;
  let mut filtered_co2_readings = filter_for_co2(&lines, bit_position);
  while filtered_co2_readings.len() > 1 {
    bit_position += 1;
    filtered_co2_readings = filter_for_co2(&filtered_co2_readings, bit_position);
  }

  let co2_reading = &filtered_co2_readings[0];
  let co2_value = isize::from_str_radix(&co2_reading, 2).unwrap();
  println!("CO2 Filtering: {} -> {}", &co2_reading, &co2_value);

  println!("Product = {}", o2_value * co2_value)

}
