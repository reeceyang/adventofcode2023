use std::{fs, collections::HashMap};

// fn main() {
//   let input = fs::read_to_string("inputs/1.txt").unwrap();
//   let mut sum = 0;
//   for line in input.split('\n') {
//     let mut num = 0;
//     for c in line.chars() {
//       if c.is_digit(10) {
//         num = 10 * c.to_digit(10).unwrap();
//         break;
//       }
//     };
//     for c in line.chars().rev() {
//       if c.is_digit(10) {
//         num += c.to_digit(10).unwrap();
//         break;
//       }
//     };
//     sum += num;
//   }
//   println!("{}", sum);
// }

fn main() {
  let numbers = HashMap::from([
      ("one", 1),
      ("two", 2),
      ("three", 3),
      ("four", 4),
      ("five", 5),
      ("six", 6),
      ("seven", 7),
      ("eight", 8),
      ("nine", 9),
      ("zero", 0),
  ]);
  
  let input = fs::read_to_string("inputs/1.txt").unwrap();
  let mut sum = 0;
  for line in input.split('\n') {
    let mut i1 = line.len();
    let mut num1 = 0;
    for (name, num) in numbers.iter() {
      match line.find(name) {
        Some(i) => if i < i1 {
          i1 = i;
          num1 = *num;
        }
        None => ()
      };
    }
    for (i, c) in line.chars().enumerate() {
      if c.is_digit(10) && i < i1 {
        num1 = c.to_digit(10).unwrap();
        break;
      }
    };

    let mut i2 = 0;
    let mut num2 = 0;
    for (name, num) in numbers.iter() {
      match line.rfind(name) {
        Some(i) => if i > i2 {
          i2 = i;
          num2 = *num;
        }
        None => ()
      };
    }
    for (i, c) in line.chars().rev().enumerate() {
      if c.is_digit(10) && line.len() - i > i2 {
        num2 = c.to_digit(10).unwrap();
        break;
      }
    };
    sum += 10 * num1 + num2;
  }
  println!("{}", sum);
}