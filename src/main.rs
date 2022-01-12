use std::env;
use std::path::PathBuf;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;


fn main() {
  let mut args = env::args();

  let mut day: Option<u32> = None;
  let mut input: Option<PathBuf> = None;

  args.next();

  loop {
    let param = args.next();
    if let None = param {
      break;
    }
    let param = param.unwrap();

    let value = args.next();
    if let None = value {
      panic!("No value provided for that parameter: {}", &param);
    }
    let value = value.unwrap();

    match &param as &str {
      "-d" | "--day" => day = Some(value.parse().unwrap()),
      "-i" | "--input" => input = Some(PathBuf::from(value)),
      _ => { panic!("Unknown parameter: {}", &param)}
    };
  }

  match day {
    Some(1) =>  day1::run(input),
    Some(2) =>  day2::run(input),
    Some(3) =>  day3::run(input),
    Some(4) =>  day4::run(input),
    Some(5) =>  day5::run(input),
    Some(6) =>  day6::run(),
    Some(7) =>  day7::run(),
    Some(8) =>  day8::run(input),
    Some(9) =>  day9::run(input),
    Some(10) => day10::run(input),
    Some(11) => day11::run(input),
    Some(12) => day12::run(input),
    Some(13) => day13::run(input),
    Some(14) => day14::run(input),
    Some(15) => day15::run(input),
    Some(16) => day16::run(),
    _ => panic!("Day {} not implemented.", day.unwrap()),
  }
}

