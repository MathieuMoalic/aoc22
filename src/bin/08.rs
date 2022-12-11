#![allow(unused_variables)]
pub fn part_one(input: &str) -> Option<u32> {
    let mut trees: Vec<Vec<i32>> = vec![];
    let mut out: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        let mut sub_trees: Vec<i32> = vec![];
        let mut sub_out: Vec<i32> = vec![];
        for char in line.chars() {
            let tree = char.to_digit(10).unwrap();
            sub_trees.push(tree.try_into().unwrap());
            sub_out.push(0);
        }
        trees.push(sub_trees);
        out.push(sub_out);
    }
    let l = trees.len() - 1;
    for i in 0..=l {
        let mut west: i32 = -1;
        let mut east: i32 = -1;
        let mut north: i32 = -1;
        let mut south: i32 = -1;
        for j in 0..=l {
            let tree = trees[i][j];
            if tree > west {
                out[i][j] = 1;
                west = tree;
            }
            let tree = trees[j][i];
            if tree > north {
                out[j][i] = 1;
                north = tree;
            }
            let tree = trees[i][l - j];
            if tree > east {
                out[i][l - j] = 1;
                east = tree;
            }
            let tree = trees[l - j][i];
            if tree > south {
                out[l - j][i] = 1;
                south = tree;
            }
        }
    }
    let mut sum: i32 = 0;
    for i in out {
        for j in i {
            sum += j
        }
    }
    Some(sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut trees: Vec<Vec<i32>> = vec![];
    let mut out: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        let mut sub_trees: Vec<i32> = vec![];
        let mut sub_out: Vec<i32> = vec![];
        for char in line.chars() {
            let tree = char.to_digit(10).unwrap();
            sub_trees.push(tree.try_into().unwrap());
            sub_out.push(0);
        }
        trees.push(sub_trees);
        out.push(sub_out);
    }
    let l = trees.len() - 1;
    // from top to bottom
    for i in 1..=l - 1 {
        // from left to right
        for j in 1..=l - 1 {
            // println!("--------------");
            let tree = trees[i][j];
            let mut scenic_score = 1;

            // to the right
            let mut nb_trees = 0;
            for x in j + 1..=l {
                nb_trees += 1;
                if trees[i][x] >= tree {
                    break;
                }
            }
            scenic_score *= nb_trees;

            // to the left
            let mut nb_trees = 0;
            for x in (0..j).rev() {
                nb_trees += 1;
                if trees[i][x] >= tree {
                    break;
                }
            }
            scenic_score *= nb_trees;

            // to the top
            let mut nb_trees = 0;
            for x in (0..i).rev() {
                nb_trees += 1;
                if trees[x][j] >= tree {
                    break;
                }
            }
            scenic_score *= nb_trees;

            // to the bottom
            let mut nb_trees = 0;
            for x in i + 1..=l {
                nb_trees += 1;
                if trees[x][j] >= tree {
                    break;
                }
            }
            scenic_score *= nb_trees;

            out[i][j] = scenic_score;
        }
    }
    // for i in &trees {
    //     println!("{:?}", i);
    // }
    // println!("---------------------");
    // for i in &out {
    //     println!("{:?}", i);
    // }
    let mut max: i32 = 0;
    for i in out {
        for j in i {
            if j > max {
                max = j;
            }
        }
    }
    Some(max.try_into().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
