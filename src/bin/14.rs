use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let max_row = input.lines().next().unwrap().len();
    let mut stones_index: Vec<u32> = vec![0; max_row];
    let max_row: u32 = max_row as u32;

    let mut r_index = 0;
    let mut res = 0;
    input.lines().for_each(|l| {
        let mut c_index = 0;
        l.chars().for_each(|c| {
            if c == 'O' {
                let old_index = stones_index[c_index];
                res += max_row - old_index;
                stones_index[c_index] += 1;
            } else if c == '#' {
                stones_index[c_index] = r_index + 1;
            }
            c_index += 1;
        });
        r_index += 1;
    });
    Some(res)
}

fn tilt_east(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut stones_2: Vec<Vec<char>> = vec![vec!['.'; input[0].len()]; input.len()];
    let mut r_index = 0;
    input.iter().for_each(|l| {
        let mut c_index = l.len() - 1;
        let mut last_pos = l.len() - 1;
        l.iter().rev().for_each(|c| {
            if *c == 'O' {
                stones_2[r_index][last_pos] = 'O';
                last_pos = last_pos.saturating_sub(1);
            } else if *c == '#' {
                stones_2[r_index][c_index] = '#';
                last_pos = c_index.saturating_sub(1);
            }
            c_index = c_index.saturating_sub(1);
        });
        r_index += 1;
    });
    stones_2
}

fn tilt_west(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut stones_2: Vec<Vec<char>> = vec![vec!['.'; input[0].len()]; input.len()];
    let mut r_index = 0;
    input.iter().for_each(|l| {
        let mut c_index = 0;
        let mut last_pos = 0;
        l.iter().for_each(|c| {
            if *c == 'O' {
                stones_2[r_index][last_pos] = 'O';
                last_pos += 1;
            } else if *c == '#' {
                stones_2[r_index][c_index] = '#';
                last_pos = c_index + 1;
            }
            c_index += 1;
        });
        r_index += 1;
    });
    stones_2
}

fn tilt_south(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut stones_2: Vec<Vec<char>> = vec![vec!['.'; input[0].len()]; input.len()];

    for col in 0..input[0].len() {
        let mut row = input.len() - 1;
        let mut last_pos = row;
        loop {
            let c = input[row][col];
            if c == 'O' {
                stones_2[last_pos][col] = 'O';
                last_pos = last_pos.saturating_sub(1);
            } else if c == '#' {
                stones_2[row][col] = '#';
                last_pos = row.saturating_sub(1);
            }
            if row == 0 {
                break;
            } else {
                row -= 1;
            }
        }
    }
    stones_2
}

fn tilt_north(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let max_row = input[0].len();
    let mut stones_2: Vec<Vec<char>> = vec![Vec::new(); max_row];
    let mut r_index = 0;
    input.iter().for_each(|l| {
        let mut c_index = 0;
        l.iter().for_each(|c| {
            if *c == 'O' {
                stones_2[c_index].push('O');
            } else if *c == '#' {
                while stones_2[c_index].len() < r_index {
                    stones_2[c_index].push('.');
                }
                stones_2[c_index].push('#');
            }
            c_index += 1;
        });
        r_index += 1;
    });
    let mut res: Vec<Vec<char>> = vec![Vec::new(); input.len()];
    for i in 0..res.len() {
        for j in 0..max_row {
            let s = &stones_2[j];
            if s.len() > i {
                res[i].push(stones_2[j][i]);
            } else {
                res[i].push('.');
            }
        }
    }
    res
}

fn tumble(input: Vec<Vec<char>>, n: u32) -> u32 {
    let mut prev_stuf = HashMap::new();
    let mut tumbled_value = input;
    let max_row = tumbled_value[0].len() as u32;

    let mut index_to_hashed = 0;
    let mut loop_length = 0;
    for index in 0..n {
        let r = tilt_east(tilt_south(tilt_west(tilt_north(tumbled_value))));
        if index_to_hashed == 0 {
            let mut hash = DefaultHasher::new();
            r.hash(&mut hash);
            let hash_of_r = hash.finish();

            let first_placed = prev_stuf.get(&hash_of_r);
            if first_placed.is_some() {
                index_to_hashed = index;
                loop_length = index - first_placed.unwrap();
            } else {
                prev_stuf.insert(hash_of_r, index);
            }
        }
        if index_to_hashed != 0 && (n - index -1) % loop_length == 0 {
            let mut r_index: u32 = 0;
            return r.iter().map(|l| {
                let r: u32 = l.iter().map(|v| {
                    if *v == 'O' {
                        max_row - r_index
                    } else {
                        0
                    }
                }).sum();
                r_index += 1;
                r
            }).sum();
        }
        tumbled_value = r;
    }
    0
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(tumble(input.lines().map(|l| l.chars().collect()).collect(), 1000000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(64));
    }
}
