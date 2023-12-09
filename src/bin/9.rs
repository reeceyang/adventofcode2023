use itertools::Itertools;
use std::fs;

// fn main() {
//     let input = fs::read_to_string("inputs/9.txt").unwrap();

//     let answer: i64 = input
//         .lines()
//         .map(|line| {
//             let seq: Vec<_> = line
//                 .split_whitespace()
//                 .map(|t| t.parse::<i64>().unwrap())
//                 .collect();
//             let mut diffs: Vec<Vec<i64>> = Vec::new();
//             diffs.push(seq);
//             while !diffs.last().unwrap().iter().all(|n| *n == 0) {
//                 let new_seq: Vec<_> = diffs
//                     .last()
//                     .unwrap()
//                     .iter()
//                     .tuple_windows()
//                     .map(|(a, b)| b - a)
//                     .collect();
//                 diffs.push(new_seq)
//             }
//             diffs
//                 .iter()
//                 .map(|v| v.last().unwrap().to_owned())
//                 .reduce(|acc, e| (acc + e))
//                 .unwrap()
//         })
//         .sum();

//     println!("{}", answer);
// }

fn main() {
    let input = fs::read_to_string("inputs/9.txt").unwrap();

    let answer: i64 = input
        .lines()
        .map(|line| {
            let seq: Vec<_> = line
                .split_whitespace()
                .map(|t| t.parse::<i64>().unwrap())
                .collect();
            let mut diffs: Vec<Vec<i64>> = Vec::new();
            diffs.push(seq);
            while !diffs.last().unwrap().iter().all(|n| *n == 0) {
                let new_seq: Vec<_> = diffs
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();
                diffs.push(new_seq)
            }
            diffs
                .iter()
                .map(|v| v.first().unwrap().to_owned())
                .rev()
                .reduce(|acc, e| (e - acc))
                .unwrap()
        })
        .sum();

    println!("{}", answer);
}
