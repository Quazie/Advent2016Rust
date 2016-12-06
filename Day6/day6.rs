// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

use std::io::prelude::*;
use std::fs::File;
use std::collections::btree_map::BTreeMap;

fn get_first_and_last(map: &BTreeMap<char, i32>) -> (char, char) {
  let mut count_vec: Vec<(&char, &i32)> = map.iter().collect();
  count_vec.sort_by(|a, b| b.1.cmp(a.1));

  (*count_vec[0].0, *count_vec.last().unwrap().0)
}

// This is the main function
fn main() {

    let mut f = File::open("day6.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let lines = s.lines();

    let mut map_vec = Vec::new();
    for _ in 0..8 {
      map_vec.push(BTreeMap::new())
    }

    for l in lines {
      let mut line = format!("{}", l);
      for i in 0..8 {
        let j = 7 - i;
        *map_vec[j].entry(line.pop().unwrap()).or_insert(0) += 1;
      }

    }

    let mut first = String::new();
    let mut last = String::new();
    for i in 0..8 {
      let f_l = get_first_and_last(&map_vec[i]);
      first += &format!("{}", f_l.0);
      last += &format!("{}", f_l.1);
    }

    println!("Part one: {:?}", first);
    println!("Part two: {:?}", last);
}
