#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Operation {
    Mult,
    Add,
    Square,
}
#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<HashSet<u32>>,
    operation_cst: u32,
    divisible_cst: u32,
    if_true: usize,
    if_false: usize,
    operation: Operation,
    inspect_count: u32,
}

impl Monkey {
    fn new(paragraph: &str) -> Self {
        let mut items = vec![];
        let mut operation_cst = 0;
        let mut divisible_cst = 0;
        let mut if_true = 0;
        let mut if_false = 0;
        let mut operation = Operation::Mult;
        for line in paragraph.lines() {
            if line.contains("Starting items:") {
                let q = line.split(": ").collect::<Vec<&str>>()[1];
                let q = q.replace(',', "");
                let nums = q.split_whitespace().collect::<Vec<&str>>();
                for item in nums {
                    let mut num: u32 = item.parse().unwrap();
                    let mut q = HashSet::new();
                    for i in 2..num {
                        if num % 2 == 0 {
                            q.insert(i);
                            num / i;
                        }
                    }
                    items.push(q);
                }
            } else if line.contains("Operation:") {
                let q = line.replace("Operation: new = old ", "");
                let q = q.split_whitespace().collect::<Vec<&str>>();
                if q[0] == "*" {
                    operation = Operation::Mult
                } else {
                    operation = Operation::Add
                }
                if q[1] == "old" {
                    operation = Operation::Square
                } else {
                    operation_cst = q[1].parse().unwrap();
                }
            } else if line.contains("Test: divisible by ") {
                let q = line.replace("  Test: divisible by ", "");
                divisible_cst = q.parse().unwrap();
            } else if line.contains("If true: ") {
                let q = line.replace("    If true: throw to monkey ", "");
                if_true = q.parse().unwrap();
            } else if line.contains("If false: ") {
                let q = line.replace("    If false: throw to monkey ", "");
                if_false = q.parse().unwrap();
            }
        }

        Self {
            items,
            operation_cst,
            divisible_cst,
            if_true,
            if_false,
            operation,
            inspect_count: 0,
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    // let mut monkeys = vec![];
    // for paragraph in input.split("\n\n") {
    //     monkeys.push(Monkey::new(paragraph));
    // }
    // for _ in 0..20 {
    //     for j in 0..monkeys.len() {
    //         let mut m = monkeys[j].clone();
    //         while !m.items.is_empty() {
    //             m.inspect_count += 1;
    //             let mut it = *m.items.first().unwrap();
    //             m.items.remove(0);
    //             match m.operation {
    //                 Operation::Mult => it *= m.operation_cst,
    //                 Operation::Add => it += m.operation_cst,
    //                 Operation::Square => it *= it,
    //             }
    //             it /= 3;
    //             if it % m.divisible_cst == 0 {
    //                 monkeys[m.if_true].items.push(it);
    //             } else {
    //                 monkeys[m.if_false].items.push(it);
    //             }
    //         }
    //         monkeys[j] = m;
    //     }
    // }
    // let mut out = vec![];
    // for m in monkeys {
    //     out.push(m.inspect_count);
    // }
    // out.sort();
    // let out: Vec<u32> = out.into_iter().rev().collect();
    // Some(out[0] * out[1])
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut monkeys = vec![];
    for paragraph in input.split("\n\n") {
        monkeys.push(Monkey::new(paragraph));
    }
    for _ in 0..10000 {
        for j in 0..monkeys.len() {
            let mut m = monkeys[j].clone();
            while !m.items.is_empty() {
                m.inspect_count += 1;
                let mut it = m.items.first().unwrap().clone();
                m.items.remove(0);
                match m.operation {
                    Operation::Mult => {
                        it.insert(m.operation_cst);
                    }
                    Operation::Add => (),
                    Operation::Square => (),
                }
                if it.contains(&m.divisible_cst) {
                    monkeys[m.if_true].items.push(it);
                } else {
                    monkeys[m.if_false].items.push(it);
                }
            }
            monkeys[j] = m;
        }
    }
    let mut out = vec![];
    for m in monkeys {
        out.push(m.inspect_count);
    }
    out.sort();
    let out: Vec<u32> = out.into_iter().rev().collect();
    Some(out[0] * out[1])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
