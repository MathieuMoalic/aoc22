#![allow(unused_variables)]

struct Cpu {
    register: i32,
    cycle: i32,
    total: i32,
    trigger_cycle: i32,
}
impl Cpu {
    fn parse_line(&mut self, line: &str) {
        if line.contains("noop") {
            self.advance_cycle()
        } else {
            let x: i32 = line.split_whitespace().collect::<Vec<&str>>()[1]
                .parse()
                .unwrap();
            self.advance_cycle();
            self.advance_cycle();
            self.register += x;
        }
    }
    fn advance_cycle(&mut self) {
        self.cycle += 1;
        if self.cycle == self.trigger_cycle {
            self.add_to_total();
        }
    }
    fn add_to_total(&mut self) {
        let signal_strength = self.cycle * self.register;
        self.total += signal_strength;
        self.trigger_cycle += 40;
    }
}
pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = Cpu {
        register: 1,
        cycle: 0,
        total: 0,
        trigger_cycle: 20,
    };
    for line in input.lines() {
        cpu.parse_line(line);
    }
    Some(cpu.total)
}

struct Crt {
    sprite_pos: i32,
    pixel_pos: i32,
}
impl Crt {
    fn parse_line(&mut self, line: &str) {
        if line.contains("noop") {
            self.draw_pixel()
        } else {
            let x: i32 = line.split_whitespace().collect::<Vec<&str>>()[1]
                .parse()
                .unwrap();
            self.draw_pixel();
            self.draw_pixel();
            self.sprite_pos += x;
        }
    }
    fn draw_pixel(&mut self) {
        if self.pixel_pos == 40 {
            self.pixel_pos = 0;
            println!();
        }
        if (self.sprite_pos - 1..=self.sprite_pos + 1).contains(&self.pixel_pos) {
            print!("#");
        } else {
            print!(" ")
        }
        self.pixel_pos += 1;
    }
}
pub fn part_two(input: &str) -> Option<i32> {
    let mut crt = Crt {
        sprite_pos: 1,
        pixel_pos: 0,
    };
    for line in input.lines() {
        crt.parse_line(line);
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
