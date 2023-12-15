use itertools::Itertools;
use std::{collections::HashMap, fs};

// fn main() {
//     let input = fs::read_to_string("inputs/14.txt").unwrap();

//     let horizontal = input
//         .lines()
//         .map(|line| line.chars().collect_vec())
//         .collect_vec();
//     let vertical = (0..horizontal[0].len())
//         .map(|col| horizontal.iter().map(|line| line[col]).collect_vec())
//         .collect_vec();

//     let answer: usize = vertical
//         .iter()
//         .map(|line| {
//             let mut weight = line.len();
//             let mut total = 0;
//             for (i, c) in line.iter().enumerate() {
//                 match c {
//                     'O' => {
//                         total += weight;
//                         weight -= 1;
//                     }
//                     '#' => {
//                         weight = line.len() - i - 1;
//                     }
//                     _ => (),
//                 }
//             }
//             total
//         })
//         .sum();
//     println!("{}", answer);
// }

fn get_north_load(vertical: Vec<Vec<char>>) -> usize {
    vertical
        .iter()
        .map(|line| {
            let mut total = 0;
            for (i, c) in line.iter().enumerate() {
                match c {
                    'O' => {
                        total += line.len() - i;
                    }
                    _ => (),
                }
            }
            total
        })
        .sum()
}

fn tilt(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_lines: Vec<Vec<char>> = Vec::new();
    for line in map {
        let mut new_line = Vec::new();
        let mut num_dots = 0;
        for c in line.iter().rev() {
            match c {
                // go through the line in reverse
                // keep track of how many empty spots to add after the O when you get to a #.
                'O' => new_line.push('O'),
                '#' => {
                    for _ in 0..num_dots {
                        new_line.push('.');
                    }
                    new_line.push('#');
                    num_dots = 0;
                }
                '.' => num_dots += 1,
                _ => panic!(),
            }
        }
        for _ in 0..num_dots {
            new_line.push('.');
        }
        new_line.reverse();
        new_lines.push(new_line);
    }
    new_lines
}

fn rot_cw_90(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..map[0].len())
        .map(|col| map.iter().map(|line| line[col]).rev().collect_vec())
        .collect_vec()
}

fn transpose(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..map[0].len())
        .map(|col| map.iter().map(|line| line[col]).collect_vec())
        .collect_vec()
}

fn main() {
    let input = fs::read_to_string("inputs/14.txt").unwrap();

    let mut horizontal = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut history_to_cycle: HashMap<Vec<Vec<char>>, i32> = HashMap::new();
    let mut cycle_to_history: HashMap<i32, Vec<Vec<char>>> = HashMap::new();
    let mut cycle = 0;
    loop {
        if history_to_cycle.contains_key(&horizontal) {
            break;
        }
        history_to_cycle.insert(horizontal.to_owned(), cycle);
        cycle_to_history.insert(cycle, horizontal.to_owned());
        cycle += 1;
        let north_east = rot_cw_90(&horizontal);
        let west_east = rot_cw_90(&tilt(&north_east));
        let south_east = rot_cw_90(&tilt(&west_east));
        let east_east = rot_cw_90(&tilt(&south_east));
        horizontal = tilt(&east_east);
    }

    let start = history_to_cycle.get(&horizontal).unwrap();

    let answer: usize = get_north_load(transpose(
        &cycle_to_history
            .get(&((1_000_000_000 - start) % (cycle - start) + start))
            .unwrap()
            .to_owned(),
    ));
    println!("{}", answer);
}
