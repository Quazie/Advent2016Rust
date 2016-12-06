use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main() {

    let mut f = File::open("day1.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
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

      match rl {
        "R" => idx = idx + 1,
        "L" => idx = idx - 1,
        _ => panic!("OMG"),
      }

      idx = ((idx % 4) + 4 ) % 4;

      let (x_dif, y_dif) = dir_info[idx as usize];
      for _ in 0..amount {
        x = x + x_dif;
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
    println!("x:{} y:{} total:{}", x, y, x.abs() + y.abs());
}
