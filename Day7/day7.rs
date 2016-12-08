use std::io::prelude::*;
use std::fs::File;
use std::collections::VecDeque;

fn get_aba_or_abba(s: &String, abba: bool) -> Vec<(char, char)>{
  let mut aba_vec = Vec::new();
  let mut char_vec = VecDeque::new();
  let len = if abba {4} else {3};

  for ch in s.chars() {
    char_vec.push_back(ch);

    // Only look at characters for the given pattern
    if char_vec.len() > len {
      char_vec.pop_front();
    }

    // if we have enough chars to look for a pattern, see if we've found the pattern
    if char_vec.len() == len {
      if char_vec[1] != char_vec[0]{
        if (abba && char_vec[0] == char_vec[3] && char_vec[1] == char_vec[2]) ||
           (!abba && char_vec[0] == char_vec[2]) {
          aba_vec.push((char_vec[0],char_vec[1]));
        }
      }
    }
  }

  return aba_vec;
}

fn main() {

  let mut f = File::open("day7.txt").unwrap();
  let mut f_s = String::new();
  f.read_to_string(&mut f_s).unwrap();

  let mut tls_count = 0; // part 1
  let mut ssl_count = 0; // part 2

  for l in f_s.lines() {
    let mut s: String = l.to_string();
    let mut supernet_bits = String::new();
    let mut hypernet_bits = String::new();

    while s.len() > 0 {
      let supernet_offset = s.find('[').unwrap_or(s.len());
      supernet_bits.extend(s.drain(..supernet_offset));

      let hypernet_offset = s.find(']').unwrap_or(s.len());
      hypernet_bits.extend(s.drain(..hypernet_offset));
    }

    if get_aba_or_abba(&supernet_bits, true).len() != 0 &&
       get_aba_or_abba(&hypernet_bits, true).len() == 0 {
      tls_count += 1;
    }

    let bab_list = get_aba_or_abba(&hypernet_bits, false);
    for aba in get_aba_or_abba(&supernet_bits, false) {
      if bab_list.contains(&(aba.1, aba.0)) {
        ssl_count += 1;
        break;
      }
    }
  }
  println!("TLS Supporting IPs: {:?}", tls_count);
  println!("SSL Supporting IPs: {:?}", ssl_count);
}
