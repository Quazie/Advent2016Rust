// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

// This is the main function
fn main() {

    let mut f = File::open("day1.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    let dirs = s.split(", ");

    let dir_info: [(i32, i32); 4] = [(0,1), (1,0), (0, -1), (-1, 0)];
    let mut idx : i32 = 0;
    let mut x:i32 = 0;
    let mut y:i32 = 0;

    let mut points = HashSet::new();
    let mut intersect_found = false;
    for dir in dirs {
      let (rl, amount_str) = dir.split_at(1);
      let amount = amount_str.parse::<i32>().unwrap();
      if rl == "R" {
        idx = idx + 1;
      } else {
        idx = idx - 1;
      }
      idx = ((idx % 4) + 4 ) % 4;
      let (x_dif, y_dif) = dir_info[idx as usize];
      let x_change = (x_dif * amount);
      let y_change = (y_dif * amount);
      if x_dif != 0 {
        for i in 0..amount {
          x = x + x_dif;
          let point = (x,y);
          if points.contains(&point) {
            if !intersect_found {
              println!("INTERSECTION x:{} y:{} total:{}", x, y, x.abs() + y.abs());
            }

            intersect_found = true
          } else {
            points.insert(point);
          }
        }
      } else {

        for i in 0..amount {
          y = y + y_dif;
          let point = (x,y);
          if points.contains(&point) {
            if !intersect_found {
              println!("INTERSECTION x:{} y:{} total:{}", x, y, x.abs() + y.abs());
            }
            intersect_found = true
          } else {
            points.insert(point);
          }
        }
      }

    }
    println!("x:{} y:{} total:{}", x, y, x.abs() + y.abs());
}
