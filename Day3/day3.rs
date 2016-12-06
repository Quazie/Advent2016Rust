// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

use std::io::prelude::*;
use std::fs::File;

// This is the main function
fn is_valid(triange: &Vec<i32>) -> bool{
  if triange.len() != 3{
    panic!("Failed to get a triange");
  }

  if triange[0] + triange[1] <= triange[2] {
    return false;
  }

  if triange[1] + triange[2] <= triange[0] {
    return false;
  }

  if triange[0] + triange[2] <= triange[1] {
    return false;
  }

  return true;
}

fn main() {

    let mut f = File::open("day3.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let lines = s.lines();

    let mut row_count = 0;
    let mut col_count = 0;
    let mut rows = Vec::new();
    for _ in 0..3 {
      rows.push(Vec::new())
    }
    for line in lines {
      let nums: Vec<i32> = line.split(' ').map(|s| s.trim())     // (2)
              .filter(|s| !s.is_empty())        // (3)
              .map(|s| s.parse().unwrap())      // (4)
              .collect();

      if is_valid(&nums) {
        row_count += 1;
      }
      for i in 0..3 {
        rows[i].push(nums[i])
      }

      if rows[0].len() == 3{
        for i in 0..3 {
          if is_valid(&rows[i]) {
            col_count += 1;
          }
          rows[i].clear();
        }
      }
    }
    println!("Num valid by row {:?}", row_count);
    println!("Num valid by col {:?}", col_count);
}
