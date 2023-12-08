use itertools::Itertools;
use num::integer::lcm;
use std::{collections::HashMap, fs};

// fn main() {
//     let input = fs::read_to_string("inputs/8.txt").unwrap();
//     let mut input_lines = input.lines();
//     let dirs: Vec<char> = input_lines.next().unwrap().chars().collect();
//     let map: HashMap<String, (String, String)> = HashMap::from_iter(input_lines.map(|l| {
//         let (k, vs) = l.split_once("=").unwrap();
//         let (left, right) = vs
//             .split(",")
//             .map(|s| s.replace("(", "").replace(")", "").trim().to_owned())
//             .next_tuple()
//             .unwrap();
//         (k.trim().to_owned(), (left, right))
//     }));

//     let mut current = "AAA".to_owned();
//     let mut steps = 0;
//     while current != "ZZZ" {
//         let options = map.get(&current).unwrap();
//         current = match dirs[steps % dirs.len()] {
//             'L' => options.0.to_owned(),
//             'R' => options.1.to_owned(),
//             _ => panic!(),
//         };
//         steps += 1;
//     }

//     println!("{}", steps);
// }

fn main() {
    let input = fs::read_to_string("inputs/8.txt").unwrap();
    let mut input_lines = input.lines();
    let dirs: Vec<char> = input_lines.next().unwrap().chars().collect();
    let map: HashMap<String, (String, String)> = HashMap::from_iter(input_lines.map(|l| {
        let (k, vs) = l.split_once("=").unwrap();
        let (left, right) = vs
            .split(",")
            .map(|s| s.replace("(", "").replace(")", "").trim().to_owned())
            .next_tuple()
            .unwrap();
        (k.trim().to_owned(), (left, right))
    }));

    let current: Vec<&String> = map.keys().filter(|k| k.ends_with("A")).collect();

    let answer = current
        .iter()
        .map(|node| {
            let mut k = *node;
            let mut steps = 0;
            while !k.ends_with("Z") {
                let options = map.get(k).unwrap();
                k = match dirs[steps % dirs.len()] {
                    'L' => &options.0,
                    'R' => &options.1,
                    _ => panic!(),
                };
                steps += 1;
            }
            steps
        })
        .reduce(|acc, e| lcm(acc, e))
        .unwrap();

    println!("{}", answer);
}
