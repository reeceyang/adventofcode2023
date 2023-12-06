use itertools::Itertools;
use std::{cmp::max, cmp::min, fs};

// fn main() {
//     let input = fs::read_to_string("inputs/5.txt").unwrap();

//     let mut input_chunks = input.split("\n\n");

//     let seeds: Vec<u64> = input_chunks
//         .next()
//         .unwrap()
//         .split_once(": ")
//         .unwrap()
//         .1
//         .split_whitespace()
//         .map(|s| s.parse().unwrap())
//         .collect();

//     type MapLine = (u64, u64, u64);

//     let mut maps: Vec<Vec<MapLine>> = Vec::new();
//     for chunk in input_chunks {
//         let mut map: Vec<MapLine> = Vec::new();
//         let chunk_lines = chunk.split_once('\n').unwrap().1;
//         chunk_lines
//             .split_whitespace()
//             .map(|s| s.parse().unwrap())
//             .tuples::<MapLine>()
//             .for_each(|map_line| map.push(map_line));
//         maps.push(map);
//     }

//     let answer = seeds
//         .iter()
//         .map(|seed| {
//             let mut mapped = *seed;
//             maps.iter().for_each(|map| {
//                 mapped = match map
//                     .iter()
//                     .filter(|map_line| mapped >= map_line.1 && mapped < map_line.1 + map_line.2)
//                     .next()
//                 {
//                     Some(map_line) => map_line.0 + mapped - map_line.1,
//                     None => mapped,
//                 };
//             });
//             mapped
//         })
//         .min()
//         .unwrap();

//     println!("{}", answer);
// }

fn main() {
    let input = fs::read_to_string("inputs/5.txt").unwrap();

    let mut input_chunks = input.split("\n\n");

    type SeedRange = (i64, i64);
    let mut seeds: Vec<SeedRange> = input_chunks
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .tuples::<(i64, i64)>()
        .collect();

    type MapLine = (i64, i64, i64);

    let mut maps: Vec<Vec<MapLine>> = Vec::new();
    for chunk in input_chunks {
        let mut map: Vec<MapLine> = Vec::new();
        let chunk_lines = chunk.split_once('\n').unwrap().1;
        chunk_lines
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .tuples::<MapLine>()
            .for_each(|map_line| map.push(map_line));
        maps.push(map);
    }

    for map in maps {
        for (mapped, start, num) in map {
            let end = start + num;
            let intersections: Vec<SeedRange> = seeds
                .iter()
                .filter(|seed| {
                    let seed_start = seed.0;
                    let seed_end = seed_start + seed.1;
                    start < seed_end && end > seed_start
                })
                .map(|s| s.to_owned())
                .collect();
            for (seed_start, seed_num) in intersections {
                let seed_end = seed_start + seed_num;
                seeds.retain(|seed| *seed != (seed_start, seed_num));
                if seed_start < start {
                    seeds.push((seed_start, start - seed_start))
                }
                seeds.push((
                    mapped + max(seed_start, start) - start,
                    min(end, seed_end) - max(seed_start, start),
                ));
                if end < seed_end {
                    seeds.push((end, seed_end - end));
                }
            }
        }
    }

    let answer = seeds.iter().min().unwrap().0;

    println!("{}", answer);
}
