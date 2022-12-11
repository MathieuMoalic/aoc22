#![allow(unused_variables)]

use std::collections::HashMap;

fn get_file_structure(input: &str) -> HashMap<Vec<String>, u32> {
    let mut dirs: HashMap<Vec<String>, u32> = HashMap::new();
    let mut pwd: Vec<String> = vec![String::from("/")];
    for line in input.lines() {
        if line.contains("$ cd") {
            if line.contains("..") {
                pwd.pop();
            } else {
                let dir = line.split_whitespace().last().unwrap();
                pwd.push(dir.to_string())
            }
        } else if line.contains("$ ls") || line.contains("dir ") {
            continue;
        } else {
            let size = line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let mut tmp = pwd.clone();
            while !tmp.is_empty() {
                *dirs.entry(tmp.clone()).or_insert(0) += size;
                tmp.pop();
            }
        }
    }
    dirs
}

pub fn part_one(input: &str) -> Option<u32> {
    let dirs = get_file_structure(input);
    let mut sum: u32 = 0;
    for (_, v) in dirs {
        if v <= 100000 {
            sum += v
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let dirs = get_file_structure(input);
    let space_needed = dirs.get(&vec![String::from("/")]).unwrap() - 40000000;
    let mut out: u32 = u32::MAX;
    for (_, size) in dirs {
        if size > space_needed && size < out {
            out = size;
        }
    }
    Some(out)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
