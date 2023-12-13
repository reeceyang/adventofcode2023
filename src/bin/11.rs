use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
};

// fn main() {
//     let input = fs::read_to_string("inputs/11.txt").unwrap();
//     let universe: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
//     let empty_rows: HashSet<_> = universe
//         .iter()
//         .enumerate()
//         .filter(|(_, line)| !line.contains(&'#'))
//         .map(|(row, _)| row)
//         .collect();
//     let empty_cols: HashSet<_> = (0..universe[0].len())
//         .filter(|col| universe.iter().all(|line| line[*col] == '.'))
//         .collect();

//     let galaxies = universe
//         .iter()
//         .enumerate()
//         .flat_map(|(row, line)| {
//             line.iter()
//                 .positions(|c| *c == '#')
//                 .map(move |col| (row, col))
//         })
//         .collect_vec();

//     let answer = galaxies
//         .iter()
//         .combinations(2)
//         .map(|combo| {
//             let mut combo_iter = combo.iter();
//             let (r1, c1) = combo_iter.next().unwrap();
//             let (r2, c2) = combo_iter.next().unwrap();

//             let empty_r = (min(*r1, *r2)..max(*r1, *r2))
//                 .filter(|r| empty_rows.contains(r))
//                 .count();
//             let empty_c = (min(*c1, *c2)..max(*c1, *c2))
//                 .filter(|c| empty_cols.contains(c))
//                 .count();

//             r1.abs_diff(*r2) + c1.abs_diff(*c2) + empty_r + empty_c
//         })
//         .sum::<usize>();

//     println!("{}", answer);
// }

fn main() {
    let input = fs::read_to_string("inputs/11.txt").unwrap();
    let universe: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let empty_rows: HashSet<_> = universe
        .iter()
        .enumerate()
        .filter(|(_, line)| !line.contains(&'#'))
        .map(|(row, _)| row)
        .collect();
    let empty_cols: HashSet<_> = (0..universe[0].len())
        .filter(|col| universe.iter().all(|line| line[*col] == '.'))
        .collect();

    let galaxies = universe
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .positions(|c| *c == '#')
                .map(move |col| (row, col))
        })
        .collect_vec();

    let answer = galaxies
        .iter()
        .combinations(2)
        .map(|combo| {
            let mut combo_iter = combo.iter();
            let (r1, c1) = combo_iter.next().unwrap();
            let (r2, c2) = combo_iter.next().unwrap();

            let empty_r = (min(*r1, *r2)..max(*r1, *r2))
                .filter(|r| empty_rows.contains(r))
                .count();
            let empty_c = (min(*c1, *c2)..max(*c1, *c2))
                .filter(|c| empty_cols.contains(c))
                .count();

            r1.abs_diff(*r2) + c1.abs_diff(*c2) + (1000000 - 1) * (empty_r + empty_c)
        })
        .sum::<usize>();

    println!("{}", answer);
}
