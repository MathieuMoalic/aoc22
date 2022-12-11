#![allow(unused_variables)]

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_points: u32 = 0;
    for line in input.lines() {
        let a = line.split_whitespace().collect::<Vec<&str>>();
        let opp = a[0];
        let me = a[1];
        if me == "X" {
            total_points += 1;
            if opp == "A" {
                total_points += 3;
            } else if opp == "C" {
                total_points += 6;
            }
        } else if me == "Y" {
            total_points += 2;
            if opp == "A" {
                total_points += 6;
            } else if opp == "B" {
                total_points += 3;
            }
        } else {
            total_points += 3;
            if opp == "B" {
                total_points += 6;
            } else if opp == "C" {
                total_points += 3;
            }
        }
    }
    Some(total_points)
}

pub fn part_two(input: &str) -> Option<u32> {
    // let table: [[u32; 3]; 3] = [
    //     [0, 0, 0], // me=rock   them=[rock, paper, scisor]
    //     [0, 0, 0], // me=paper  them=[rock, paper, scisor]
    //     [0, 0, 0], // me=scisor them=[rock, paper, scisor]
    // ];
    let mut total_points: u32 = 0;
    for line in input.lines() {
        let a = line.split_whitespace().collect::<Vec<&str>>();
        let opp = a[0];
        let me = a[1];
        // if I need to lose
        if me == "X" {
            // and my opp has rock
            if opp == "A" {
                // I play scisor
                total_points += 3;
                // and my opp has paper
            } else if opp == "B" {
                // I play rock
                total_points += 1;
            } else {
                // I play paper
                total_points += 2;
            }
        // I need to draw
        } else if me == "Y" {
            total_points += 3;
            // They play rock
            if opp == "A" {
                // I play rock
                total_points += 1;
            } else if opp == "B" {
                // I play paper
                total_points += 2;
            } else {
                // I play scisor
                total_points += 3;
            }
            // I need to win
        } else {
            total_points += 6;
            // they play rocke
            if opp == "A" {
                // I play paper
                total_points += 2;
            } else if opp == "B" {
                total_points += 3;
            } else {
                total_points += 1
            }
        }
    }
    Some(total_points)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
