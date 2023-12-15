use itertools::Itertools;
use std::fs;

fn get_reflection_line(pat: Vec<Vec<char>>, ignore: usize) -> Result<usize, ()> {
    for i in 0..(pat.len() - 1) {
        if pat
            .iter()
            .enumerate()
            .skip(i + 1)
            .take(i + 1)
            .all(|(j, line)| *line == pat[2 * (i + 1) - j - 1])
            && i + 1 != ignore
        {
            return Ok(i + 1);
        }
    }
    Err(())
}

// fn main() {
//     let input = fs::read_to_string("inputs/13.txt").unwrap();

//     let answer = input
//         .split("\n\n")
//         .map(|pat| {
//             let horizontal = pat
//                 .lines()
//                 .map(|line| line.chars().collect_vec())
//                 .collect_vec();
//             let vertical = (0..horizontal[0].len())
//                 .map(|col| horizontal.iter().map(|line| line[col]).collect_vec())
//                 .collect_vec();
//             match get_reflection_line(horizontal) {
//                 Ok(i) => 100 * i,
//                 Err(_) => get_reflection_line(vertical).unwrap(),
//             }
//         })
//         .sum::<usize>();

//     println!("{}", answer);
// }

const LARGE_NUMBER: usize = 999;

fn main() {
    let input = fs::read_to_string("inputs/13.txt").unwrap();

    let answer = input
        .split("\n\n")
        .map(|pat| {
            let horizontal = pat
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec();
            let vertical = (0..horizontal[0].len())
                .map(|col| horizontal.iter().map(|line| line[col]).collect_vec())
                .collect_vec();
            let old_answer = match get_reflection_line(horizontal, LARGE_NUMBER) {
                Ok(i) => 100 * i,
                Err(_) => get_reflection_line(vertical, LARGE_NUMBER).unwrap(),
            };
            pat.char_indices()
                .filter(|(_, c)| *c != '\n')
                .map(|(i, c)| {
                    let new_c = if c == '.' { '#' } else { '.' };
                    let new_pat = format!(
                        "{}{}{}",
                        pat.chars().take(i).join(""),
                        new_c,
                        pat.chars().skip(i + 1).join("")
                    );
                    let horizontal = new_pat
                        .lines()
                        .map(|line| line.chars().collect_vec())
                        .collect_vec();
                    let vertical = (0..horizontal[0].len())
                        .map(|col| horizontal.iter().map(|line| line[col]).collect_vec())
                        .collect_vec();
                    match get_reflection_line(horizontal, old_answer / 100) {
                        Ok(i) => Ok(100 * i),
                        Err(_) => get_reflection_line(vertical, old_answer),
                    }
                })
                .filter(|r| r.is_ok_and(|v| v != old_answer))
                .map(|r| r.unwrap())
                .next()
                .unwrap()
        })
        .sum::<usize>();

    println!("{}", answer);
}
