use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

// fn main() {
//     let input = fs::read_to_string("inputs/3.txt").unwrap();
//     let re_num = Regex::new(r"\d+").unwrap();
//     let re_symbol = Regex::new(r"[^.\d]").unwrap();
//     let mut symbol_indices: HashSet<(i32, i32)> = HashSet::new();
//     for (line_idx, line) in input.lines().enumerate() {
//         for symbol_match in re_symbol.find_iter(line) {
//             symbol_indices.insert((
//                 line_idx.try_into().unwrap(),
//                 symbol_match.start().try_into().unwrap(),
//             ));
//         }
//     }
//     let mut part_sum = 0;
//     for (line_idx, line) in input.lines().enumerate() {
//         for num_match in re_num.find_iter(line) {
//             let mut found = false;
//             for i in num_match.range() {
//                 for dr in (-1..2).collect::<Vec<i32>>() {
//                     for dc in (-1..2).collect::<Vec<i32>>() {
//                         let r: i32 = line_idx.try_into().unwrap();
//                         let c: i32 = i.try_into().unwrap();
//                         if symbol_indices.contains(&(r + dr, c + dc)) {
//                             found = true;
//                         }
//                     }
//                 }
//             }
//             if found {
//                 part_sum += num_match.as_str().parse::<i32>().unwrap();
//             }
//         }
//     }
//     println!("{}", part_sum)
// }

fn main() {
    let input = fs::read_to_string("inputs/3.txt").unwrap();
    let re_num = Regex::new(r"\d+").unwrap();
    let re_symbol = Regex::new(r"\*").unwrap();
    let mut symbol_indices: HashMap<(i32, i32), HashSet<i32>> = HashMap::new();
    for (line_idx, line) in input.lines().enumerate() {
        for symbol_match in re_symbol.find_iter(line) {
            symbol_indices.insert(
                (
                    line_idx.try_into().unwrap(),
                    symbol_match.start().try_into().unwrap(),
                ),
                HashSet::new(),
            );
        }
    }
    for (line_idx, line) in input.lines().enumerate() {
        for num_match in re_num.find_iter(line) {
            for i in num_match.range() {
                for dr in (-1..2).collect::<Vec<i32>>() {
                    for dc in (-1..2).collect::<Vec<i32>>() {
                        let r: i32 = line_idx.try_into().unwrap();
                        let c: i32 = i.try_into().unwrap();
                        let key = (r + dr, c + dc);
                        if symbol_indices.contains_key(&key) {
                            symbol_indices
                                .get_mut(&key)
                                .unwrap()
                                .insert(num_match.as_str().parse::<i32>().unwrap());
                        }
                    }
                }
            }
        }
    }
    println!(
        "{}",
        symbol_indices
            .values()
            .filter(|s| s.len() == 2)
            .map(|s| s.iter().product::<i32>())
            .sum::<i32>()
    )
}
