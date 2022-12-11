#![allow(unused_variables)]

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let (q1, q2) = line.split_at(line.len() / 2);
        for char in q1.chars() {
            if q2.contains(char) {
                let priority = char as u8;
                if (65..=90).contains(&priority) {
                    total += priority as u32 - 38
                } else {
                    total += priority as u32 - 96
                }
                break;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for i in (0..lines.len()).step_by(3) {
        for char in lines[i].chars() {
            if lines[i + 1].contains(char) && lines[i + 2].contains(char) {
                let priority = char as u8;
                if (65..=90).contains(&priority) {
                    total += priority as u32 - 38
                } else {
                    total += priority as u32 - 96
                }
                break;
            }
        }
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_03_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
