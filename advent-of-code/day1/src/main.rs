use std::io;
use std::path::Path;
use std::fs;

pub fn read_input_and_format<P: AsRef<Path>>(
    cfg_file: P,
) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(&cfg_file).unwrap();
    let entries = input.lines();
    let mut vec1 : Vec<i32> = vec![];
    let mut vec2 : Vec<i32> = vec![];
    for e in entries {
      let mut columns = e.split_whitespace();
      let num1 = columns.next().unwrap().parse().unwrap();
      let num2 = columns.next().unwrap().parse().unwrap();

      vec1.push(num1);
      vec2.push(num2);
    }

    return (vec1, vec2);
}


fn main() {
  let (mut vec1, mut vec2) = read_input_and_format("./input.txt");

  vec1.sort();
  vec2.sort();
  
  let mut count = 0;
  let mut simScore = 0;

  for i in 0..vec1.len() {
    let diff = (vec1[i] - vec2[i]).abs();
    count += diff;

    let num = vec1[i];
    let mut count = 0;
    for j in 0..vec2.len() {
      if vec2[j] == num {
        count += 1;
      }
    }
    simScore += num * count;
  }
  println!("diffCount: {}", count);
  println!("simScore: {}", simScore);
}