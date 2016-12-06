// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

// This is the main function
fn is_valid(triange: &Vec<i32>) -> bool{
  if triange.len() != 3{
    panic!("Failed to get a triange");
  }

  if(triange[0] + triange[1] <= triange[2]) {
    return false;
  }

  if(triange[1] + triange[2] <= triange[0]) {
    return false;
  }

  if(triange[0] + triange[2] <= triange[1]) {
    return false;
  }

  return true;
}

fn main() {

    let mut f = File::open("day3.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    let lines = s.lines();
    let mut row_count = 0;
    let mut col_count = 0;
    let mut row1: Vec<i32> = Vec::new();
    let mut row2: Vec<i32> = Vec::new();
    let mut row3: Vec<i32> = Vec::new();
    for line in lines {
      let nums: Vec<i32> = line.split(' ').map(|s| s.trim())     // (2)
              .filter(|s| !s.is_empty())        // (3)
              .map(|s| s.parse().unwrap())      // (4)
              .collect();

      if(is_valid(&nums)){
        row_count += 1;
      }
      row1.push(nums[0]);
      row2.push(nums[1]);
      row3.push(nums[2]);
      if row1.len() == 3{
        if(is_valid(&row1)){
          col_count += 1;
        }
        row1.clear();
        if(is_valid(&row2)){
          col_count += 1;
        }
        row2.clear();
        if(is_valid(&row3)){
          col_count += 1;
        }
        row3.clear();

      }
    }
    println!("Num valid by row {:?}", row_count);
    println!("Num valid by col {:?}", col_count);
}
