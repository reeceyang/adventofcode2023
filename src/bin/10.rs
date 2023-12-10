use itertools::Itertools;
use std::fs;

// fn main() {
//     let input = fs::read_to_string("inputs/10.txt").unwrap();
//     let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

//     let sr = map.iter().position(|l| l.contains(&'S')).unwrap();
//     let sc = map[sr].iter().position(|c| c == &'S').unwrap();

//     let mut r = sr;
//     let mut c = sc;
//     let mut steps = 0;
//     let mut visited: HashSet<(usize, usize)> = HashSet::new();
//     loop {
//         visited.insert((r, c));
//         match map[r][c] {
//             // S is a J, let's go left
//             'S' => c -= 1,
//             '|' => {
//                 if visited.contains(&(r + 1, c)) {
//                     r -= 1;
//                 } else {
//                     r += 1;
//                 }
//             }
//             '-' => {
//                 if visited.contains(&(r, c + 1)) {
//                     c -= 1;
//                 } else {
//                     c += 1;
//                 }
//             }
//             'L' => {
//                 if visited.contains(&(r, c + 1)) {
//                     r -= 1;
//                 } else {
//                     c += 1;
//                 }
//             }
//             'J' => {
//                 if visited.contains(&(r, c - 1)) {
//                     r -= 1;
//                 } else {
//                     c -= 1;
//                 }
//             }
//             '7' => {
//                 if visited.contains(&(r, c - 1)) {
//                     r += 1;
//                 } else {
//                     c -= 1;
//                 }
//             }
//             'F' => {
//                 if visited.contains(&(r, c + 1)) {
//                     r += 1;
//                 } else {
//                     c += 1;
//                 }
//             }
//             _ => panic!(),
//         }
//         steps += 1;
//         if (r, c) == (sr, sc) {
//             break;
//         }
//     }

//     println!("{}", steps / 2);
// }

fn main() {
    let input = fs::read_to_string("inputs/10.txt").unwrap();
    let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let sr = map.iter().position(|l| l.contains(&'S')).unwrap();
    let sc = map[sr].iter().position(|c| c == &'S').unwrap();

    let mut r = sr;
    let mut c = sc;
    let mut visited: Vec<(usize, usize)> = Vec::new();
    loop {
        visited.push((r, c));
        match map[r][c] {
            // S is a J, let's go left
            'S' => c -= 1,
            '|' => {
                if visited.contains(&(r + 1, c)) {
                    r -= 1;
                } else {
                    r += 1;
                }
            }
            '-' => {
                if visited.contains(&(r, c + 1)) {
                    c -= 1;
                } else {
                    c += 1;
                }
            }
            'L' => {
                if visited.contains(&(r, c + 1)) {
                    r -= 1;
                } else {
                    c += 1;
                }
            }
            'J' => {
                if visited.contains(&(r, c - 1)) {
                    r -= 1;
                } else {
                    c -= 1;
                }
            }
            '7' => {
                if visited.contains(&(r, c - 1)) {
                    r += 1;
                } else {
                    c -= 1;
                }
            }
            'F' => {
                if visited.contains(&(r, c + 1)) {
                    r += 1;
                } else {
                    c += 1;
                }
            }
            _ => panic!(),
        }
        if (r, c) == (sr, sc) {
            break;
        }
    }

    let mut area = 0;
    for i in 0..map.len() {
        println!("{}/{}", i, map.len());
        for j in 0..map[i].len() {
            if visited.contains(&(i, j)) {
                continue;
            };
            let left = (0..j)
                .filter(|cj| visited.contains(&(i, *cj)) && map[i][*cj] != '-')
                .map(|cj| map[i][cj])
                .join("");
            let intersections = left.matches("|").count()
                + left.matches("L7").count()
                + left.matches("FJ").count()
                + 2 * (left.matches("LJ").count() + left.matches("F7").count());
            if intersections % 2 == 1 {
                area += 1;
            }
        }
    }

    println!("{}", area);
}
