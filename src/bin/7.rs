use itertools::Itertools;
use std::{cmp::Ordering, fs};

// fn main() {
//     let input = fs::read_to_string("inputs/7.txt").unwrap();

//     let num_order = vec![
//         'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
//     ];

//     let answer: usize = input
//         .lines()
//         .map(|l| {
//             let (hand, bid) = l.split_once(' ').unwrap();
//             (
//                 hand.chars().collect::<Vec<char>>(),
//                 bid.parse::<usize>().unwrap(),
//             )
//         })
//         .sorted_by(|(a, _), (b, _)| {
//             let a_counts = a.into_iter().counts();
//             let b_counts = b.into_iter().counts();
//             if a_counts.len() < b_counts.len() {
//                 Ordering::Less
//             } else if b_counts.len() < a_counts.len() {
//                 Ordering::Greater
//             } else {
//                 if a_counts.values().max() > b_counts.values().max() {
//                     Ordering::Less
//                 } else if b_counts.values().max() > a_counts.values().max() {
//                     Ordering::Greater
//                 } else {
//                     let (a_str, b_str) = vec![a, b]
//                         .iter()
//                         .map(|h| {
//                             h.iter()
//                                 .map(|c| {
//                                     char::from_digit(
//                                         num_order
//                                             .iter()
//                                             .position(|&cc| cc == *c)
//                                             .unwrap()
//                                             .try_into()
//                                             .unwrap(),
//                                         16,
//                                     )
//                                     .unwrap()
//                                 })
//                                 .join("")
//                         })
//                         .next_tuple()
//                         .unwrap();
//                     a_str.cmp(&b_str)
//                 }
//             }
//         })
//         .rev()
//         .enumerate()
//         .map(|(i, (_, bid))| (i + 1) * bid)
//         .sum();

//     println!("{}", answer);
// }

fn main() {
    let input = fs::read_to_string("inputs/7.txt").unwrap();

    let num_order = vec![
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    let answer: usize = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            (
                hand.chars().collect::<Vec<char>>(),
                bid.parse::<usize>().unwrap(),
            )
        })
        .sorted_by(|(a, _), (b, _)| {
            let mut a_counts = a.into_iter().counts();
            if a_counts.contains_key(&'J') && a_counts.len() > 1 {
                let j_val = a_counts.remove(&'J').unwrap();
                let max_char = a_counts
                    .iter()
                    .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
                    .unwrap()
                    .0;
                a_counts.insert(&max_char, a_counts[max_char] + j_val);
            }
            let mut b_counts = b.into_iter().counts();
            if b_counts.contains_key(&'J') && b_counts.len() > 1 {
                let j_val = b_counts.remove(&'J').unwrap();
                let max_char = b_counts
                    .iter()
                    .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
                    .unwrap()
                    .0;
                b_counts.insert(&max_char, b_counts[max_char] + j_val);
            }
            if a_counts.len() < b_counts.len() {
                Ordering::Less
            } else if b_counts.len() < a_counts.len() {
                Ordering::Greater
            } else {
                if a_counts.values().max() > b_counts.values().max() {
                    Ordering::Less
                } else if b_counts.values().max() > a_counts.values().max() {
                    Ordering::Greater
                } else {
                    let (a_str, b_str) = vec![a, b]
                        .iter()
                        .map(|h| {
                            h.iter()
                                .map(|c| {
                                    char::from_digit(
                                        num_order
                                            .iter()
                                            .position(|&cc| cc == *c)
                                            .unwrap()
                                            .try_into()
                                            .unwrap(),
                                        16,
                                    )
                                    .unwrap()
                                })
                                .join("")
                        })
                        .next_tuple()
                        .unwrap();
                    // println!("{} {}", a.iter().join(""), b.iter().join(""));
                    // println!("{} {}", a_str, b_str);
                    a_str.cmp(&b_str)
                }
            }
        })
        .rev()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum();

    println!("{}", answer);
}
