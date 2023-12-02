use regex::Regex;
use std::{collections::HashMap, fs};

// fn main() {
//     let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
//     let re_game = Regex::new(r"Game(?P<game>\d+)").unwrap();
//     let re_draw = Regex::new(r"(\d+\w+,)+;").unwrap();
//     let re_cube = Regex::new(r"(?<num>\d+)(?<color>\w+)").unwrap();
//     let input = fs::read_to_string("inputs/2.txt").unwrap();
//     let mut id_sum: i32 = 0;
//     for line in input.lines() {
//         let line_edited = (line.to_owned() + ";").replace(" ", "").replace(";", ",;");
//         let Some(game_caps) = re_game.captures(line_edited.as_str()) else {
//             panic!("no match!");
//         };
//         let mut possible: bool = true;
//         for draw in re_draw.find_iter(line_edited.as_str()) {
//             for cube in re_cube.find_iter(draw.as_str()) {
//                 let Some(caps) = re_cube.captures(cube.as_str()) else {
//                     panic!("no match!")
//                 };
//                 for (color, limit) in limits.iter() {
//                     if *color == &caps["color"] && &caps["num"].parse::<i32>().unwrap() > limit {
//                         possible = false;
//                         break;
//                     }
//                 }
//             }
//             if !possible {
//                 break;
//             }
//         }
//         if possible {
//             id_sum += &game_caps["game"].parse::<i32>().unwrap();
//         }
//     }
//     println!("{}", id_sum);
// }

fn main() {
    let re_draw = Regex::new(r"(\d+\w+,)+;").unwrap();
    let re_cube = Regex::new(r"(?<num>\d+)(?<color>\w+)").unwrap();
    let input = fs::read_to_string("inputs/2.txt").unwrap();
    let mut power_sum: i32 = 0;
    for line in input.lines() {
        let line_edited = (line.to_owned() + ";").replace(" ", "").replace(";", ",;");
        let mut maxes: HashMap<String, i32> = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);
        for draw in re_draw.find_iter(line_edited.as_str()) {
            for cube in re_cube.find_iter(draw.as_str()) {
                let Some(caps) = re_cube.captures(cube.as_str()) else {
                    panic!("no match!")
                };
                let num = &caps["num"].parse::<i32>().unwrap();
                let color = &caps["color"];
                if num > &maxes[&caps["color"]] {
                    maxes.insert(color.to_string(), *num);
                }
            }
        }
        let mut power = 1;
        for num in maxes.values() {
            power *= num;
        }
        power_sum += power;
    }
    println!("{}", power_sum);
}
