pub fn part_one(input: &str) -> Option<u32> {
    let mut max_cal: u32 = 0;
    for grp in input.split("\n\n") {
        let mut grp_cal: u32 = 0;
        for food in grp.split('\n') {
            let cal: Result<u32, _> = food.parse();
            if let Ok(cal) = cal {
                grp_cal += cal
            }
        }
        if grp_cal > max_cal {
            max_cal = grp_cal
        }
    }
    Some(max_cal)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut max_cal: [u32; 4] = [0, 0, 0, 0];
    for grp in input.split("\n\n") {
        let mut grp_cal: u32 = 0;
        for food in grp.split('\n') {
            let cal: Result<u32, _> = food.parse();
            if let Ok(cal) = cal {
                grp_cal += cal
            }
        }
        max_cal[0] = grp_cal;
        max_cal.sort()
    }
    Some(max_cal[1..4].iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
