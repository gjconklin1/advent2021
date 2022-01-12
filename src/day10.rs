use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;


pub fn run(input: Option<PathBuf>) {
  let file = File::open(input.unwrap()).expect("Unable to open file.");
  let reader = BufReader::new(file);

  let mut total_corruption_score = 0;
  let mut completion_scores: Vec<i64> = Vec::new();

  for line in reader.lines() {
    let line = line.unwrap();

    let mut chunk_stack: Vec<char> = Vec::new();

    let mut corruption_score = 0;
    for c in line.chars() {
      match c {
        '(' => {chunk_stack.push(c);},
        '[' => {chunk_stack.push(c);},
        '{' => {chunk_stack.push(c);},
        '<' => {chunk_stack.push(c);},

        ')' => {
          match chunk_stack.pop() {
            Some(e) => { if e != '(' { corruption_score = 3 } },
            None => ()
          }
        },
        ']' => {
          match chunk_stack.pop() {
            Some(e) => { if e != '[' { corruption_score = 57 } },
            None => ()
          }
        },
        '}' => {
          match chunk_stack.pop() {
            Some(e) => { if e != '{' { corruption_score = 1197 } },
            None => ()
          }
        },
        '>' => {
          match chunk_stack.pop() {
            Some(e) => { if e != '<' { corruption_score = 25137 } },
            None => ()
          }
        },
 
        _ => (),
      };

      if corruption_score > 0 {
        total_corruption_score += corruption_score;
        break;
      }
    }

    if corruption_score == 0 {
      let mut completion_score: i64 = 0;
      for token in chunk_stack.iter().rev() {
        completion_score *= 5;
        completion_score += match token {
          '(' => 1,
          '[' => 2,
          '{' => 3,
          '<' => 4,
          _ => 0
        };
      }
      completion_scores.push(completion_score);
    }
  }

  completion_scores.sort();

  println!("Part 1: Total Corruption Score = {}", total_corruption_score);
  println!("Part 2: Median Completion Score = {}", completion_scores[(completion_scores.len() - 1) / 2]);

}
