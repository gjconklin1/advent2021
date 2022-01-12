use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn run(input: Option<PathBuf>) {
  // Open the file and get a reader
  let file = File::open(input.unwrap()).expect("Unable to open file.");
  let reader = BufReader::new(file);

  let mut edges: Vec<(String, String)> = Vec::new();

  // Pull out the first line and parse into numbers
  for line in reader.lines() {
    let line = line.expect("Cannot read line.");
    let line_split: Vec<&str> = line.split("-").collect();
    edges.push((String::from(line_split[0]), String::from(line_split[1])));
  }

  // Accumulate paths here
  let mut all_paths: Vec<Vec<String>> = Vec::new();

  // Find paths...
  let path_so_far = vec![];
  let paths = find_paths(&edges, "start", &path_so_far);
  for path in paths {
    all_paths.push(path);
  }

  println!("{} paths found...", all_paths.len());
  
  if all_paths.len() < 50 {
    for path in all_paths {
      println!(" {:?}", path);
    }  
  }
  
}


fn find_paths(edges: &Vec<(String, String)>, start_edge: &str, path_so_far: &Vec<&str>) -> Vec<Vec<String>> {

//  println!("path_so_far = {:?}", path_so_far);
  let mut paths: Vec<Vec<String>> = Vec::new();

  for edge in edges {
    let mut next_node: Option<&str> = None; 
    if edge.0.eq(start_edge) {
      next_node = Some(&edge.1);
    }
    if edge.1.eq(start_edge) {
      next_node = Some(&edge.0);
    }

    if let Some(next_node) = next_node {
      if next_node.eq("end") {
        let mut path: Vec<String> = Vec::new();
        path.push(String::from(start_edge));
        path.push(String::from(next_node));
        paths.push(path);
      }
      else {
        let mut this_path_so_far = path_so_far.to_vec();
        this_path_so_far.push(&start_edge);
        if can_proceed(&this_path_so_far, &next_node) {

          let other_paths = find_paths(&edges, &next_node, &this_path_so_far);

          for other_path in other_paths {
            if other_path.len() > 0 {
              let mut path: Vec<String> = Vec::new();
              path.push(String::from(start_edge));
              for node in other_path {
                path.push(String::from(node));
              }
              paths.push(path);
            }
          }  
        }
      }      
    }
  }

  paths
}


fn can_proceed(path_so_far: &Vec<&str>, next_node: &str) -> bool {

  if next_node.chars().next().unwrap().is_uppercase() {
    return true;
  }

  if next_node.eq("start") {
    return false;
  }

  let mut count_of_doubles = 0;
  let mut count_of_next_node = 0;

  for i in 0..path_so_far.len() {
    if path_so_far[i].chars().next().unwrap().is_uppercase() {
      continue;
    }
    if path_so_far[i].eq(next_node) {
      count_of_next_node += 1;
    }
    for j in i + 1..path_so_far.len() {
      if path_so_far[i].eq(path_so_far[j]) {
        if path_so_far[i].eq(next_node) {
          return false;
        }
        count_of_doubles += 1;
      }
    }
  }

  if count_of_next_node > 0 && count_of_doubles > 0  {
    return false;
  }

  true
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_can_proceed() {
    assert!(!can_proceed(&vec!["start", "a"], "start"));
    assert!( can_proceed(&vec!["start", "a"], "a"));
    assert!(!can_proceed(&vec!["start", "a", "a"], "a"));
    assert!( can_proceed(&vec!["start", "a", "a"], "b"));
    assert!( can_proceed(&vec!["start", "a", "b"], "a"));
    assert!(!can_proceed(&vec!["start", "a", "a", "b"], "b"));
    assert!( can_proceed(&vec!["start", "A", "b", "A", "b"], "c"));
    assert!( can_proceed(&vec!["start", "A", "b", "A", "b"], "A"));
  }
}
