use std::cmp::{max, min};
advent_of_code::solution!(11);

fn explode_space(input: &str) -> Vec<Vec<char>> {
    let mut space: Vec<Vec<char>> = Vec::new();
    let mut explod_col: Vec<bool> = Vec::new();

    let mut row = 0;
    input.lines().for_each(|l| {
        if explod_col.is_empty() {
            l.chars().for_each(|_| explod_col.push(true));
        }

        let mut r_explode = true;
        let mut space_row = Vec::new();
        let mut col = 0;
        l.chars().for_each(|c| {
            if c == '#' {
                r_explode = false;
                explod_col[col] = false;
            }
            col += 1;
            space_row.push(c);
        });

        if r_explode {
            space.push(space_row.clone());
        }
        space.push(space_row);
        row += 1;
    });
    space.iter().map(|row| {
        let mut exploded_col: Vec<char> = Vec::new();
        let mut col = 0;
        row.iter().for_each(|v| {
            exploded_col.push(*v);
            if explod_col[col] {
                exploded_col.push('.');
            }
            col += 1;
        });
        exploded_col
    }).collect()
}


#[derive(Debug, Clone, Copy)]
struct LNode {
    row: u32,
    col: u32,
}

fn diff(i: u32, o: u32) -> u32 {
    if i < o {
        o - i
    } else {
        i - o
    }
}

fn diff_u64(i: u64, o: u64) -> u64 {
    if i < o {
        o - i
    } else {
        i - o
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let space = explode_space(input);
    let mut stars = Vec::new();
    let mut row = 0;
    space.iter().for_each(|l| {
        let mut col = 0;
        l.iter().for_each(|c| {
            if *c == '#' {
                stars.push(LNode { row, col });
            }
            col += 1;
        });
        row += 1;
    });
    let mut sum = 0;
    for i in 0..stars.len() {
        for j in i + 1..stars.len() {
            sum += diff(stars[i].row, stars[j].row) + diff(stars[i].col, stars[j].col)
        }
    }

    Some(sum)
}

const EXPANSE_LENGTH: u64 = 1000000;

fn diff_v2(i: u64, o: u64, expanse: Vec<bool>) -> u64 {
    let mut org_diff = diff_u64(i, o);
    for j in min(i, o)..max(o, i) {
        if expanse[j as usize] {
            org_diff += EXPANSE_LENGTH - 1;
        }
    }
    org_diff
}
/*

fn diff_v2(i: u64, o: u64, expanse_s: u32, expanse_e: u32) -> u64 {
    diff_u64(i, o) + (EXPANSE_LENGTH * diff(expanse_s, expanse_e)) as u64
}
 */

pub fn part_two(input: &str) -> Option<u64> {
    let mut space: Vec<Vec<char>> = Vec::new();
    let mut explod_col: Vec<bool> = Vec::new();
    let mut explod_row: Vec<bool> = Vec::new();

    let mut stars = Vec::new();
    let mut row = 0;
    input.lines().for_each(|l| {
        if explod_col.is_empty() {
            l.chars().for_each(|_| explod_col.push(true));
        }
        explod_row.push(true);
        let mut space_row = Vec::new();
        let mut col = 0;
        l.chars().for_each(|c| {
            if c == '#' {
                explod_row[row] = false;
                explod_col[col] = false;
                // TODO iter stars here
                stars.push(LNode {
                    row: row as u32,
                    col: col as u32,
                });
            }
            col += 1;
            space_row.push(c);
        });
        space.push(space_row);
        row += 1;
    });


    let mut sum: u64 = 0;
    for i in 0..stars.len() {
        for j in i + 1..stars.len() {
            sum += diff_v2(stars[i].row as u64, stars[j].row as u64, explod_row.clone()) + diff_v2(stars[i].col as u64, stars[j].col as u64, explod_col.clone())
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
