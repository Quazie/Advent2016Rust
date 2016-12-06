// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::cmp;

fn move_dir(dir: char, move_info: &[i32], pos: (i32, i32), base: i32) -> (i32, i32) {
  let (mut x, mut y) = pos;

  match(dir) {
    'U' => y = cmp::max(base - move_info[x as usize], y-1),
    'D' => y = cmp::min(base + move_info[x as usize], y + 1),
    'L' => x = cmp::max(base - move_info[y as usize], x-1),
    'R' => x = cmp::min(base + move_info[y as usize], x + 1),
    _ => panic!("OMG"),
  }

  (x, y)
}

// This is the main function
fn main() {

    let mut f = File::open("day2.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    let lines = s.split("\n");
    let min = 0;
    let max = 2;
    let size_info1: [i32; 3] = [1, 1, 1];
    let mut pos1 = (1, 1);
    let mut pos2 = (2, 2);
    let size_info2: [i32; 5] = [0,1,2,1,0];
    let mut ans1 = String::new();
    let mut ans2 = String::new();
    for line in lines {
      for ch in line.chars() {
        pos1 = move_dir(ch, &size_info1[..], pos1, 1);
        pos2 = move_dir(ch, &size_info2[..], pos2, 2);
      }

      let (x, y) = pos1;
      let number = 1 + x + 3 * y;
      ans1 += &format!("{}",number);

      let (x, y) = pos2;
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
      ans2 += &format!("{:x}", number);
    }
    println!("Bathroom Code 1: {}", ans1);
    println!("Bathroom Code 2: {}", ans2);
}
