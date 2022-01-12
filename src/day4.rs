use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::path::PathBuf;


/*
  A board is represented as a HashMap of number -> (x,y) position
*/

pub struct BoardState {
  rows: HashMap<u32, u32>,
  cols: HashMap<u32, u32>,
  remaining_sum: u32,
  has_won: bool,
}

pub fn load_input_file(input: PathBuf) -> (Vec<u32>, Vec<HashMap<u32, (u32, u32)>>) {
  // Open the file and get a reader
  let file = File::open(input).expect("Unable to open file.");
  let mut reader = BufReader::new(file);

  // Pull out the first line and parse into numbers
  let mut draw_numbers: Vec<u32> = Vec::new();
  let mut first_line = String::new();
  reader.read_line(&mut first_line).unwrap();
  let draw_split: Vec<&str> = first_line.trim().split(",").collect();
  for token in draw_split {
    draw_numbers.push(token.parse().unwrap());
  }

  // Boards
  let mut boards: Vec<HashMap<u32, (u32, u32)>> = Vec::new();
  let mut board: HashMap<u32, (u32, u32)> = HashMap::new();

  let mut y_pos: u32 = 0;
  for (_file_idx, read_line) in reader.lines().enumerate() {
    let line = read_line.expect("Problem reading line!");
    if line.chars().count() == 0 {
      // If we've filled out a board then add it to `boards`
      // and start a new `board`
      if board.keys().len() > 0 {
        boards.push(board);
        board = HashMap::new();
        y_pos = 0;
      }
    }
    else {
      // Add these numbers to the board
      let board_split: Vec<&str> = line.split_whitespace().collect();
      let mut x_pos = 0;
      for token in board_split {
        board.insert(token.parse().unwrap(), (x_pos, y_pos));
        x_pos += 1;
      }
      y_pos += 1
    }      
  }
  // Add the last board to the boards
  if board.keys().len() > 0 {
    boards.push(board);
  }

  (draw_numbers, boards)
}

#[allow(mutable_borrow_reservation_conflict)]
pub fn run(input: Option<PathBuf>) {
  // Load file
  let (draw_numbers, boards) = load_input_file(input.unwrap());

  // Initialize board states
  let mut board_states: Vec<BoardState> = Vec::new();
  for board in &boards {
    let board_state = BoardState {
      rows: HashMap::new(),
      cols: HashMap::new(),
      remaining_sum: board.keys().sum(),
      has_won: false,
    };
    board_states.push(board_state)
  }

  // Play game
  for draw_number in &draw_numbers {
    // Update all boards
    let mut board_num = 0;
    for board in &boards {
      let board_state = &mut board_states[board_num];
      if board_state.has_won == false {
        match board.get(&draw_number) {
          None => (),
          Some((x_pos, y_pos)) => {
            // Update board state
            board_state.remaining_sum -= *draw_number;

            match board_state.rows.get(x_pos) {
              None => {
                board_state.rows.insert(*x_pos, 1);
              },
              Some(i) => {
                if *i == 4 {
                  let score = board_state.remaining_sum * *draw_number;
                  board_state.has_won = true;
                  println!("Winner on board {}! [score: {}]", board_num, score);
                }
                board_state.rows.insert(*x_pos, i+1);
              }
            };

            match board_state.cols.get(y_pos) {
              None => {
                board_state.cols.insert(*y_pos, 1);
              },
              Some(i) => {
                if *i == 4 {
                  let score = board_state.remaining_sum * *draw_number;
                  board_state.has_won = true;
                  println!("Winner on board {}! [score: {}]", board_num, score);
                }
                board_state.cols.insert(*y_pos, i+1);
              }
            }
          }
        }
      }

      board_num += 1;
    }

  }



}
