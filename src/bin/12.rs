use itertools::Itertools;
use std::{collections::HashMap, fs, iter};

fn get_num_arrangements(
    memo: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
    record: Vec<char>,
    groups: Vec<usize>,
) -> usize {
    let memo_key = (record.clone(), groups.clone());
    if memo.contains_key(&memo_key) {
        return memo[&memo_key];
    }
    if groups.len() == 0 {
        if record.contains(&'#') {
            memo.insert(memo_key, 0);
            return 0;
        }
        memo.insert(memo_key, 1);
        return 1;
    }
    let group = groups[0];
    if group == 0 {
        if record.len() == 0 {
            if groups.len() == 1 {
                memo.insert(memo_key, 1);
                return 1;
            }
            memo.insert(memo_key, 0);
            return 0;
        } else if record[0] != '#' {
            return get_num_arrangements(memo, record[1..].to_vec(), groups[1..].to_vec());
        }
        memo.insert(memo_key, 0);
        return 0;
    }
    if record.len() == 0 {
        memo.insert(memo_key, 0);
        return 0;
    }
    let next_num = if record[0] == '#' {
        0
    } else {
        get_num_arrangements(memo, record[1..].to_vec(), groups.clone())
    };
    if record
        .iter()
        .take(group)
        .join("")
        .replace("?", "#")
        .matches("#")
        .count()
        == group
    {
        let mut new_groups = vec![0 as usize];
        new_groups.append(&mut groups[1..].to_vec());
        let total = next_num + get_num_arrangements(memo, record[group..].to_vec(), new_groups);
        memo.insert(memo_key, total);
        return total;
    }
    memo.insert(memo_key, next_num);
    return next_num;
}

// fn main() {
//     let input = fs::read_to_string("inputs/12.txt").unwrap();

//     let answer = input
//         .lines()
//         .map(|line| {
//             let (record_str, groups_str) = line.split_once(" ").unwrap();
//             get_num_arrangements(
//                 record_str.chars().collect_vec(),
//                 groups_str
//                     .split(",")
//                     .map(|c| c.parse::<usize>().unwrap())
//                     .collect_vec(),
//             )
//         })
//         .sum::<usize>();

//     println!("{}", answer);
// }

fn main() {
    let input = fs::read_to_string("inputs/12.txt").unwrap();

    let answer = input
        .lines()
        .map(|line| {
            let (record_str, groups_str) = line.split_once(" ").unwrap();
            let unfolded_str = iter::repeat(record_str).take(5).join("?");
            let mut memo = HashMap::new();
            get_num_arrangements(
                &mut memo,
                unfolded_str.chars().collect_vec(),
                groups_str
                    .split(",")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect_vec()
                    .repeat(5),
            )
        })
        .sum::<usize>();

    println!("{}", answer);
}
