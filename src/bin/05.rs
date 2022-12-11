#![allow(unused_variables)]

use std::str::Lines;
fn parse_crate_line(line: &str) -> Vec<Option<char>> {
    let mut v = vec![];
    let mut chars = line.chars();
    loop {
        if chars.next().is_none() {
            break;
        }
        let c = chars.next().unwrap();
        chars.next();
        if c.is_alphabetic() {
            v.push(Some(c));
        } else {
            v.push(None);
        }
        chars.next();
    }
    v
}
fn get_stacks(lines: &mut Lines) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![];
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() || line.contains('1') {
            lines.next();
            break;
        }
        let crates = parse_crate_line(line);
        if stacks.is_empty() {
            for _ in 0..crates.len() {
                stacks.push(vec![])
            }
        }
        for (i, c) in crates.iter().enumerate() {
            match c {
                Some(c) => stacks[i].insert(0, *c),
                None => (),
            }
        }
    }
    stacks
}
fn get_moves(lines: &mut Lines) -> Vec<(usize, usize, usize)> {
    let mut moves = vec![];
    loop {
        let line = lines.next();
        if let Some(line) = line {
            let line = line.replace("move ", "");
            let line = line.replace("from ", "");
            let line = line.replace(" to", "");
            let nbs: Vec<usize> = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let count = nbs[0];
            let src = nbs[1] - 1;
            let dst = nbs[2] - 1;
            moves.push((count, src, dst))
        } else {
            break;
        }
    }
    moves
}
pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let mut stacks = get_stacks(&mut lines);
    let moves = get_moves(&mut lines);
    for (count, src, dst) in moves {
        for _ in 0..count {
            let crt = stacks[src].pop().unwrap();
            stacks[dst].push(crt)
        }
    }
    Some(stacks.iter().map(|stack| stack.last().unwrap()).collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let mut stacks = get_stacks(&mut lines);
    let moves = get_moves(&mut lines);
    for (count, src, dst) in moves {
        let mut tmp_vec = vec![];
        for _ in 0..count {
            let crt = stacks[src].pop().unwrap();
            tmp_vec.push(crt);
        }
        for _ in 0..count {
            let crt = tmp_vec.pop().unwrap();
            stacks[dst].push(crt)
        }
    }
    Some(stacks.iter().map(|stack| stack.last().unwrap()).collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
