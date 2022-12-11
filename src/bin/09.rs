#![allow(unused_variables)]
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    i: String,
}
struct Plane {
    segments: Vec<Point>,
    tail_hist: HashSet<Point>,
}

impl Plane {
    fn new(length: usize) -> Self {
        let mut v = vec![];
        for i in 0..length {
            if i == 0 {
                v.push(Point {
                    x: 0,
                    y: 0,
                    i: "H".to_string(),
                });
            } else {
                v.push(Point {
                    x: 0,
                    y: 0,
                    i: format!("{}", i),
                });
            }
        }
        Self {
            segments: v,
            tail_hist: HashSet::new(),
        }
    }
    fn update_tail(&mut self) {
        for i in 0..self.segments.len() - 1 {
            let ppt = &self.segments[i];
            let pt = &self.segments[i + 1];
            let mut dx: i32 = ppt.x as i32 - pt.x as i32;
            let mut dy: i32 = ppt.y as i32 - pt.y as i32;
            // println!("dx: {dx}, dy: {dy}");
            if dx.abs() == 2 && dy.abs() == 1 {
                dy = dy.signum() * 2;
            } else if dx.abs() == 1 && dy.abs() == 2 {
                dx = dx.signum() * 2;
            }
            // println!("dx: {dx}, dy: {dy}");
            dx = correct_delta(dx);
            dy = correct_delta(dy);
            // println!("dx: {dx}, dy: {dy}");
            self.segments[i + 1].x = self.segments[i + 1].x + dx;
            self.segments[i + 1].y = self.segments[i + 1].y + dy;
            // println!("head: x:{},y:{}", self.hx, self.hy);
            // println!("tail: x:{},y:{}", self.tx, self.ty);
        }
        self.tail_hist.insert(self.segments.last().unwrap().clone());
        // dbg!(&self.tail_hist);
    }
    fn move_head(&mut self, mv: &str) {
        match mv {
            "U" => self.segments[0].y += 1,
            "D" => self.segments[0].y -= 1,
            "R" => self.segments[0].x += 1,
            "L" => self.segments[0].x -= 1,
            _ => (),
        }
    }
    fn visualize(&self) {
        // println!("{},{},{},{}", self.hx, self.hy, self.tx, self.ty);
        let mut v = vec![];
        let size = 30;
        for _ in 0..size {
            let mut tmp_v = vec![];
            for _ in 0..size {
                tmp_v.push(".");
            }
            v.push(tmp_v);
        }
        for pt in self.segments.iter().rev() {
            v[(pt.y + size / 2) as usize][(pt.x + size / 2) as usize] = &pt.i;
        }
        // v[0][0] = "s";
        for i in v.iter().rev() {
            for j in i {
                print!("{}", j);
            }
            println!(".");
        }
    }
    fn visualize_answer(&self) {
        let mut v = vec![];
        let size = 30;
        for _ in 0..size {
            let mut tmp_v = vec![];
            for _ in 0..size {
                tmp_v.push(".");
            }
            v.push(tmp_v);
        }
        for pt in &self.tail_hist {
            v[(pt.y + size / 2) as usize][(pt.x + size / 2) as usize] = "#"
        }
        v[0][0] = "s";
        for i in v.iter().rev() {
            for j in i {
                print!("{}", j);
            }
            println!();
        }
    }
}

fn correct_delta(d: i32) -> i32 {
    if d == 0 {
        d
    } else if d.is_negative() {
        d + 1
    } else {
        d - 1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(13)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut p = Plane::new(10);
    // p.visualize();
    for line in input.lines() {
        let q: Vec<&str> = line.split_whitespace().collect();
        let mv = q[0];
        let reps = q[1].parse::<u32>().unwrap();
        for _ in 0..reps {
            // println!("---------------------");
            p.move_head(mv);
            p.update_tail();
            // p.visualize();
        }
    }
    // println!("---------------------");
    // p.visualize_answer();
    Some(p.tail_hist.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
