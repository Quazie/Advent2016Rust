use std::io::prelude::*;
use std::fs::File;

const MAX_X: usize = 50;
const MAX_Y: usize = 6;

fn print_board(board: &Vec<Vec<bool>>) {

  let mut count = 0;
  for i in 0..MAX_Y {
    let mut line = String::new();;
    for j in 0..MAX_X {
      if board[i][j] {
        line += "#";
        count += 1;
      } else {
        line += ".";
      }
    }

    println!("{}", line);
  }
  println!("\n{} pixels lit", count);
}

fn add_rect(board: &mut Vec<Vec<bool>>, x: usize, y: usize)
{
  for i in 0..y {
    for j in 0..x {
      board[i][j] = true;
    }
  }

}

fn rotate_row(board: &mut Vec<Vec<bool>>, y: usize)
{
  let mut val = board[y][0];
  for x in 1..MAX_X {
    let temp = board[y][x];
    board[y][x] = val;
    val = temp;
  }
  board[y][0] = val;
}

fn rotate_column(board: &mut Vec<Vec<bool>>, x: usize)
{
  let mut val = board[0][x];
  for y in 1..MAX_Y {
    let temp = board[y][x];
    board[y][x] = val;
    val = temp;
  }
  board[0][x] = val;
}

fn main() {

  let mut f = File::open("day8.txt").unwrap();
  let mut f_s = String::new();
  f.read_to_string(&mut f_s).unwrap();

  let mut board = vec![vec![false; 50]; 6];
  for l in f_s.lines() {
    let line = l.to_string();
    if line.contains("rect") {
      let (_, last) = l.split_at(5);
      let mut last_str = last.to_string();
      let cross_pos = last_str.find("x").unwrap();
      last_str.remove(cross_pos);
      let (x, y) = last_str.split_at(cross_pos);
      add_rect(&mut board, x.parse().unwrap(), y.parse().unwrap())

    } else if line.contains("column") {
      let (_, last) = l.split_at(16);
      let mut last_str = last.to_string();
      let by_pos = last_str.find(" by").unwrap();

      for _ in 0..4{
        last_str.remove(by_pos);
      }
      let (x, times) = last_str.split_at(by_pos);

      for _ in 0..times.parse().unwrap() {
        rotate_column(&mut board, x.parse().unwrap())
      }

    } else if line.contains("row") {
      let (_, last) = l.split_at(13);
      let mut last_str = last.to_string();
      let by_pos = last_str.find(" by").unwrap();

      for _ in 0..4{
        last_str.remove(by_pos);
      }
      let (y, times) = last_str.split_at(by_pos);

      for _ in 0..times.parse().unwrap() {
        rotate_row(&mut board, y.parse().unwrap())
      }

    }
  }
  print_board(&board);

}
