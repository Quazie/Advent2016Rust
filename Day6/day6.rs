// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::collections::btree_map::BTreeMap;
use std::cmp::Ordering;

fn get_first_and_last(map: BTreeMap<char, i32>) -> (char, char) {
  let mut count_vec: Vec<(&char, &i32)> = map.iter().collect();
  count_vec.sort_by(|a, b| b.1.cmp(a.1));

  (*count_vec[0].0, *count_vec.last().unwrap().0)
}

// This is the main function
fn main() {

    let mut f = File::open("day6.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    let lines = s.lines();
    let mut count0 = BTreeMap::new();
    let mut count1 = BTreeMap::new();
    let mut count2 = BTreeMap::new();
    let mut count3 = BTreeMap::new();
    let mut count4 = BTreeMap::new();
    let mut count5 = BTreeMap::new();
    let mut count6 = BTreeMap::new();
    let mut count7 = BTreeMap::new();
    for l in lines {
      let mut line = String::new();
      line = format!("{}", l);
      *count7.entry(line.pop().unwrap()).or_insert(0) += 1;
      *count6.entry(line.pop().unwrap()).or_insert(0) += 1;
      *count5.entry(line.pop().unwrap()).or_insert(0) += 1;
      *count4.entry(line.pop().unwrap()).or_insert(0) += 1;
      *count3.entry(line.pop().unwrap()).or_insert(0) += 1;
      *count2.entry(line.pop().unwrap()).or_insert(0) += 1;
      *count1.entry(line.pop().unwrap()).or_insert(0) += 1;
      *count0.entry(line.pop().unwrap()).or_insert(0) += 1;
    }

    let mut first = String::new();
    let mut last = String::new();
    let mut fl = (' ', ' ');
    fl = get_first_and_last(count0);
    first += &format!("{}", fl.0);
    last += &format!("{}", fl.1);
    fl = get_first_and_last(count1);
    first += &format!("{}", fl.0);
    last += &format!("{}", fl.1);
    fl = get_first_and_last(count2);
    first += &format!("{}", fl.0);
    last += &format!("{}", fl.1);
    fl = get_first_and_last(count3);
    first += &format!("{}", fl.0);
    last += &format!("{}", fl.1);
    fl = get_first_and_last(count4);
    first += &format!("{}", fl.0);
    last += &format!("{}", fl.1);
    fl = get_first_and_last(count5);
    first += &format!("{}", fl.0);
    last += &format!("{}", fl.1);
    fl = get_first_and_last(count6);
    first += &format!("{}", fl.0);
    last += &format!("{}", fl.1);
    fl = get_first_and_last(count7);
    first += &format!("{}", fl.0);
    last += &format!("{}", fl.1);

    println!("Part one: {:?}", first);
    println!("Part two: {:?}", last);
}
