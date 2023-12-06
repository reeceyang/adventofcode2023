use itertools::Itertools;
use std::fs;

// fn main() {
//     let input = fs::read_to_string("inputs/6.txt").unwrap();
//     let mut input_iter = input.lines();

//     let times: Vec<i32> = input_iter
//         .next()
//         .unwrap()
//         .split_once(":")
//         .unwrap()
//         .1
//         .trim()
//         .split_whitespace()
//         .map(|s| s.parse().unwrap())
//         .collect();

//     let distances: Vec<i32> = input_iter
//         .next()
//         .unwrap()
//         .split_once(":")
//         .unwrap()
//         .1
//         .trim()
//         .split_whitespace()
//         .map(|s| s.parse().unwrap())
//         .collect();

//     let answer: usize = zip(times, distances)
//         .map(|(time, distance)| (0..time).filter(|n| n * (time - n) > distance).count())
//         .product();

//     println!("{}", answer);
// }

fn main() {
    let input = fs::read_to_string("inputs/6.txt").unwrap();
    let mut input_iter = input.lines();

    let time = input_iter
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .join("")
        .parse::<u64>()
        .unwrap();

    let distance = input_iter
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .join("")
        .parse::<u64>()
        .unwrap();

    let answer: usize = (0..time).filter(|n| n * (time - n) > distance).count();

    println!("{}", answer);
}
