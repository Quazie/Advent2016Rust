// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::cmp;

// This is the main function
fn main() {

    let mut f = File::open("day2.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    let lines = s.split("\n");
    let min = 0;
    let max = 2;
    let mut pos = (1, 1);
    println!("Answer 1");
    for line in lines {
      for ch in line.chars() {
        let (mut x, mut y) = pos;
        if ch == 'U' {
          y = cmp::max(0, y-1);

        } else if ch == 'D' {
          y = cmp::min(2, y + 1);
        } else if ch == 'L' {
          x = cmp::max(0, x-1);
        } else if ch == 'R' {
          x = cmp::min(2, x + 1);

        }
        pos = (x, y);
      }
      let (x, y) = pos;
      let number = 1 + x + 3 * y;
      println!("{:?}", number);
    }

    println!("Answer 2");
    let mut pos = (2, 2);
    let size_info: [i32; 5] = [0,1,2,1,0];
    let lines = s.split("\n");
    for line in lines {
      for ch in line.chars() {
        let (mut x, mut y) = pos;
        if ch == 'U' {
          y = cmp::max(2 - size_info[x as usize], y-1);

        } else if ch == 'D' {
          y = cmp::min(2 + size_info[x as usize], y + 1);
        } else if ch == 'L' {
          x = cmp::max(2 - size_info[y as usize], x-1);
        } else if ch == 'R' {
          x = cmp::min(2 + size_info[y as usize], x + 1);

        }
        pos = (x, y);
      }
      let (x, y) = pos;
      let mut number = 0;
      if y == 0 {
        number = 1;
      } else if y == 1 {
        number = 1 + x
      } else if y == 2 {
        number = 5 + x
      } else if y == 3 {
        number = 9 + x
      } else {
        number = 13
      }
      println!("{:x}", number);
    }
}
