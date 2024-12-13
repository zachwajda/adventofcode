use std::path::Path;
use std::fs;
use regex::Regex;

pub fn read_input_and_format<P: AsRef<Path>>(
    cfg_file: P,
) -> String {
    let input = fs::read_to_string(&cfg_file).unwrap();

    return input;
}

fn main() {
  let mut input = read_input_and_format("./input.txt");

  let re = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)").unwrap();
  let do_re = Regex::new(r"(do[^n])").unwrap();  
  let dont_re = Regex::new(r"(don\'t)").unwrap();

  let regex_match = re.captures_iter(&input);

  let mut count = 0;
  for capture in regex_match {
    let index = capture.get(0).unwrap();
    let mut closest_do = 0;
    let do_matches = do_re.captures_iter(&input);
    for do_match in do_matches {
      let do_index = do_match.get(0).unwrap();
      let diff: i32 = (do_index.start() as i32) - (index.start() as i32);
      if diff < 0 && (diff.abs() < closest_do || closest_do == 0) {
        closest_do = diff.abs();
      } else if diff > 0 {
        break;
      }
    }
    
    let mut closest_dont = 0; 
    let dont_matches = dont_re.captures_iter(&input);
    for dont_match in dont_matches {
      let dont_index = dont_match.get(0).unwrap();
      let diff: i32 = dont_index.start() as i32 - index.start() as i32;
      if diff < 0 && (diff.abs() < closest_dont || closest_dont == 0) {
        closest_dont = diff.abs();
      } else if diff > 0 {
        break;
      }
    }

    if closest_do < closest_dont  || closest_dont == 0 {
      let first_op : u32 = capture.get(1).unwrap().as_str().parse().unwrap();
      let second_op : u32 = capture.get(2).unwrap().as_str().parse().unwrap();
      count += first_op * second_op;
    }
  }
  println!("count: {}", count);
}