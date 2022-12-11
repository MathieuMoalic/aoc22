#![allow(unused_variables)]
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let q: Vec<u32> = line
            .split(',')
            .flat_map(|s| s.split('-').collect::<Vec<&str>>())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        if (q[0] <= q[2] && q[1] >= q[3]) || (q[0] >= q[2] && q[1] <= q[3]) {
            sum += 1
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| {
                line.split(&['-', ','])
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .map(|q| if !(q[1] < q[2] || q[3] < q[0]) { 1 } else { 0 })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
