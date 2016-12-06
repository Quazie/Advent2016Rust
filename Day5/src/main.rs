

// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;



// This is the main function
fn main() {
    let mut password = String::new();

    let mut hasher = Md5::new();
    let key = "ugkcyxxp".as_bytes();
    let mut password2: Vec<char> = vec![' '; 8];
    for i in 0..u64::max_value(){
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = hasher.result_str();

        if output.starts_with("00000"){
          let char7 = output.remove(6);
          let char6 = output.remove(5);
          let index:i32 = i32::from_str_radix(&char6.to_string(), 16).unwrap();
          if password.len() != 8 {
            password += &format!("{}", char6);
            println!("Password 1 is: {:?}", password);
          }
          if index < 8  && password2[index as usize] == ' '{
            password2[index as usize] = char7;
            println!("Password 2 is: {:?}", password2);
          }
          let mut done = true;
          for j in 0..8 {
            if password2[j as usize] == ' ' {
              done = false;
              break;
            }
          }
          if done {
            break;
          }
        }

        hasher.reset();
    }

    println!("Password 1 is: {:?}", password);
    println!("Password 2 is: {:?}", password2);
}
