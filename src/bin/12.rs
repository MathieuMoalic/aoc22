#![allow(unused_variables)]

fn get_map(input: &str) -> Vec<Vec<u8>> {
    let mut map = vec![];
    for line in input.lines() {
        let mut v = vec![];
        for char in line.chars() {
            if char == 'S' {
                v.push(0)
            } else if char == 'E' {
                v.push(27)
            } else {
                v.push(char as u8 - 96)
            }
        }
        map.push(v);
    }
    map
}

#[derive(PartialEq, Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}
struct Path {
    pos: Point,
    past_pos: Vec<Point>,
    size: usize,
}
impl Path {
    fn get_adjacent(self) -> (Vec<Point>, Vec<Point>) {
        let mut out = vec![];
        for x in [-1, 0, 1] {
            let i = self.pos.x as i32 - x;
            if i < 0 || i >= self.size as i32 {
                continue;
            }
            for y in [-1, 0, 1] {
                let j = self.pos.y as i32 - y;
                if j < 0 || j >= self.size as i32 {
                    continue;
                }
                let p = Point {
                    x: i as usize,
                    y: j as usize,
                };
                if !self.past_pos.contains(&p) {
                    out.push(p);
                }
            }
        }
        (out, self.past_pos)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = get_map(input);
    let mut paths = vec![Path {
        pos: Point { x: 0, y: 0 },
        past_pos: vec![Point { x: 0, y: 0 }],
        size: map.len(),
    }];
    while !paths.is_empty() {
        let p = paths.remove(0);
        let (mut adj, mut past_pos) = p.get_adjacent();
        let height = map[pt.y][pt.x];
        while !adj.is_empty() {
            let pt = adj.remove(0);
            if height == 27 {
                println!("WE WON !!");
                println!("{:?}", pt);
            } else {
                past_pos.push(pt.clone());
                paths.push(Path {
                    pos: pt.clone(),
                    past_pos: past_pos.to_vec(),
                    size: map.len(),
                })
            }
        }
        println!("{:?}", adj);
    }

    for i in map {
        println!("{:?}", i);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
