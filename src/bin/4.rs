use std::{collections::HashSet, fs};

// fn main() {
//     let input = fs::read_to_string("inputs/4.txt").unwrap();

//     let answer: u32 = input
//         .lines()
//         .map(|line| -> u32 {
//             let (win_nums, my_nums) = line.split_once(':').unwrap().1.split_once('|').unwrap();
//             let win_set: HashSet<&str> = HashSet::from_iter(win_nums.split_whitespace());
//             let my_set: HashSet<&str> = HashSet::from_iter(my_nums.split_whitespace());
//             win_set.intersection(&my_set).count().try_into().unwrap()
//         })
//         .filter(|n| n > &0)
//         .map(|n| u32::pow(2, n - 1))
//         .sum();

//     println!("{}", answer)
// }

fn main() {
    let input = fs::read_to_string("inputs/4.txt").unwrap();
    let matches: Vec<u32> = input
        .lines()
        .map(|line| -> u32 {
            let (win_nums, my_nums) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let win_set: HashSet<&str> = HashSet::from_iter(win_nums.split_whitespace());
            let my_set: HashSet<&str> = HashSet::from_iter(my_nums.split_whitespace());
            let num_matches = win_set.intersection(&my_set).count().try_into().unwrap();
            num_matches
        })
        .collect();
    let mut total_copies: u32 = 0;
    let mut num_copies = vec![1; matches.len()];
    for i in 0..matches.len() {
        let upper_bound: usize = matches[i].try_into().unwrap();
        for j in i + 1..i + upper_bound + 1 {
            num_copies[j] += num_copies[i];
        }
        total_copies += num_copies[i];
    }

    println!("{}", total_copies)
}
