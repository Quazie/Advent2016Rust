use std::io::prelude::*;
use std::fs::File;
use std::collections::btree_map::BTreeMap;
use std::cmp::Ordering;

fn main() {

  let mut f = File::open("day4.txt").unwrap();
  let mut s = String::new();
  f.read_to_string(&mut s).unwrap();
  let lines = s.lines();

  let mut amount:i32 = 0;
  let ascii_iter = (0..26).map(|x| (x + 'a' as u8) as char);
  let ascii_vec = ascii_iter.collect::<Vec<_>>();
  for line in lines {
    let mut is_checksum = false;
    let mut checksum = String::new();
    let mut str_id = String::new();
    let mut to_test = String::new();
    for ch in line.chars() {
      if ch == '[' {
        is_checksum = true;
      } else if ch == '-' || ch == ']' {
        continue;
      } else if ch.is_digit(10) {
        str_id += &format!("{}",ch);
      } else if is_checksum {
        checksum += &format!("{}",ch);
      } else {
        to_test += &format!("{}",ch);
      }
    }
    let mut count = BTreeMap::new();
    for c in to_test.chars() {
      *count.entry(c).or_insert(0) += 1;
    }
    let mut count_vec: Vec<(&char, &u32)> = count.iter().collect();
    count_vec.sort_by(|a, b| if b.1.cmp(a.1) == Ordering::Equal { a.0.cmp(b.0) }else{ b.1.cmp(a.1)});
    let mut checksum_test = String::new();
    for i in 0..5 {
      checksum_test += &format!("{}",count_vec[i].0);
    }

    if checksum_test == checksum {
      let sector_num = str_id.parse().unwrap();

      // update the count for part 1
      amount += sector_num;

      // generate part 2 output
      let mut to_print = String::new();
      for c in to_test.chars() {
        let mut index:i32 = ascii_vec.iter().position(|&r| r == c).unwrap() as i32;
        index += sector_num;
        index = index % 26;
        to_print += &format!("{}",ascii_vec[index as usize]);
      }

      if to_print == "northpoleobjectstorage" {
        println!("North Pole Objects in sector: {}", sector_num);
      }

    }
  }
  println!("The sum of all valid room sectors is: {:?}",amount );




}
