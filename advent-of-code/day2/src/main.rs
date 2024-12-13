use std::path::Path;
use std::fs;

pub fn read_input_and_format<P: AsRef<Path>>(
    cfg_file: P,
) -> Vec<Vec<i32>> {
    let input = fs::read_to_string(&cfg_file).unwrap();
    let entries = input.lines();
    let mut reports : Vec<Vec<i32>> = vec![];
    for e in entries {
      let columns = e.split_whitespace();
      let mut levels : Vec<i32> = vec![];
      for c in columns {
        levels.push(c.parse().unwrap());
      }
      reports.push(levels);
    }

    return reports;
}

pub fn check_safety(report: &Vec<i32>) -> bool {
  let first_diff = report[1] - report[0];
  let direction = first_diff > 0;

  for l in 1..report.len() {
    let diff = report[l] - report[l-1];
    if diff.abs() < 1 || diff.abs() > 3 || (diff > 0) != direction {
      return false;
    }
  }
  return true; 
}

fn main() {
  let mut reports = read_input_and_format("./input.txt");

  let mut safe_count = 0;
  for report in reports {
    let safe = check_safety(&report);
    if safe == true {
      safe_count += 1;
    } else {
      for i in 0..report.len() {
        let mut temp_report = report.clone();
        temp_report.remove(i);
        if check_safety(&temp_report) == true {
          safe_count += 1;
          break;
        }
      }
    }
  }

  println!("safeCount: {}", safe_count);
}