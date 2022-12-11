#![allow(unused_variables)]
pub fn part_one(input: &str) -> Option<u32> {
    let mut v: Vec<char> = vec![];
    for (i, char) in input.chars().enumerate() {
        if v.len() > 3 {
            let mut tmp_v = v.clone();
            tmp_v.sort();
            tmp_v.dedup();
            if tmp_v.len() == 4 {
                return Some(i as u32);
            }
            v.remove(0);
            v.push(char);
        } else {
            v.push(char)
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut v: Vec<char> = vec![];
    for (i, char) in input.chars().enumerate() {
        if v.len() > 13 {
            let mut tmp_v = v.clone();
            tmp_v.sort();
            tmp_v.dedup();
            if tmp_v.len() == 14 {
                return Some(i as u32);
            }
            v.remove(0);
            v.push(char);
        } else {
            v.push(char)
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
