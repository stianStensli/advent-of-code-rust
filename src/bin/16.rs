use crate::Direction::{Down, Left, Rigth, Up};
use std::cmp::max;
advent_of_code::solution!(16);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Rigth,
}

struct Light {
    dir: Direction,
    c: usize,
    r: usize,
}

fn update_res_board(l: &Light, res: &Vec<Direction>) -> Vec<Direction> {
    let mut new_res = res.clone();
    if !new_res.contains(&l.dir) {
        new_res.push(l.dir)
    }
    new_res
}

fn get_dir_node(l: &Light, next_dir: Direction, max_r: usize, max_c: usize) -> Option<Light> {
    match next_dir {
        Up => {
            if l.r == 0 {
                return None;
            }
            Some(Light {
                dir: Up,
                c: l.c,
                r: l.r - 1,
            })
        }
        Down => {
            if l.r + 1 == max_r {
                return None;
            }
            Some(Light {
                dir: Down,
                c: l.c,
                r: l.r + 1,
            })
        }
        Left => {
            if l.c == 0 {
                return None;
            }
            Some(Light {
                dir: Left,
                c: l.c - 1,
                r: l.r,
            })
        }
        Rigth => {
            if l.c + 1 == max_c {
                return None;
            }
            Some(Light {
                dir: Rigth,
                c: l.c + 1,
                r: l.r,
            })
        }
    }
}

fn main_part(board: &Vec<Vec<char>>, start: Light) -> u64 {
    let mut res: u64 = 0;
    let mut res_board: Vec<Vec<Vec<Direction>>> =
        vec![vec![Vec::new(); board[0].len()]; board.len()];
    let mut stack: Vec<Light> = vec![start];

    let max_r = board.len();
    let max_c = board[0].len();
    while let Some(current) = stack.pop() {
        let mut new_ligth: Vec<Light> = Vec::new();
        let b_v = board[current.r][current.c];
        /*
        println!(
            "current: {}, dir: {:?}, r: {}, c: {}",
            b_v, current.dir, current.r, current.c
        );
        prin");
        res_board.iter().for_each(|l| {
            l.iter().for_each(|v| {
                if v.is_empty() {
                    print!(".")
                } else {
                    print!("#")
                }
            });
            print!("\n");
        });*/
        match current.dir {
            Rigth => {
                if b_v == '|' {
                    if let Some(opt) = get_dir_node(&current, Down, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                    if let Some(opt) = get_dir_node(&current, Up, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if b_v == '\\' {
                    if let Some(opt) = get_dir_node(&current, Down, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if b_v == '/' {
                    if let Some(opt) = get_dir_node(&current, Up, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if let Some(opt) = get_dir_node(&current, Rigth, max_r, max_c) {
                    new_ligth.push(opt)
                }
            }
            Left => {
                if b_v == '|' {
                    if let Some(opt) = get_dir_node(&current, Down, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                    if let Some(opt) = get_dir_node(&current, Up, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if b_v == '\\' {
                    if let Some(opt) = get_dir_node(&current, Up, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if b_v == '/' {
                    if let Some(opt) = get_dir_node(&current, Down, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if let Some(opt) = get_dir_node(&current, Left, max_r, max_c) {
                    new_ligth.push(opt)
                }
            }
            Up => {
                if b_v == '-' {
                    if let Some(opt) = get_dir_node(&current, Left, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                    if let Some(opt) = get_dir_node(&current, Rigth, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if b_v == '/' {
                    if let Some(opt) = get_dir_node(&current, Rigth, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if b_v == '\\' {
                    if let Some(opt) = get_dir_node(&current, Left, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if let Some(opt) = get_dir_node(&current, Up, max_r, max_c) {
                    new_ligth.push(opt)
                }
            }
            Down => {
                if b_v == '-' {
                    if let Some(opt) = get_dir_node(&current, Left, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                    if let Some(opt) = get_dir_node(&current, Rigth, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if b_v == '/' {
                    if let Some(opt) = get_dir_node(&current, Left, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if b_v == '\\' {
                    if let Some(opt) = get_dir_node(&current, Rigth, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                } else if let Some(opt) = get_dir_node(&current, Down, max_r, max_c) {
                    new_ligth.push(opt)
                }
            }
        }
        let r_old = &res_board[current.r][current.c];
        let r_old_length = r_old.len();

        let new_res = update_res_board(&current, r_old);
        if r_old_length == 0 {
            res += 1;
        }
        if r_old_length < new_res.len() {
            while let Some(l) = new_ligth.pop() {
                stack.push(l)
            }
        }
        res_board[current.r][current.c] = new_res;
    }

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut board: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|l| {
        if l.is_empty() {
            return;
        }
        let mut row = Vec::new();
        l.chars().for_each(|c| row.push(c));
        board.push(row);
    });
    Some(main_part(
        &board,
        Light {
            dir: Rigth,
            c: 0,
            r: 0,
        },
    ))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut max_res = 0;
    let mut board: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|l| {
        if l.is_empty() {
            return;
        }
        let mut row = Vec::new();
        l.chars().for_each(|c| row.push(c));
        board.push(row);
    });
    for i in 0..board.len() {
        let t_res = main_part(
            &board,
            Light {
                dir: Down,
                c: i,
                r: 0,
            },
        );
        let b_res = main_part(
            &board,
            Light {
                dir: Up,
                c: i,
                r: board.len() - 1,
            },
        );
        max_res = max(max(b_res, t_res), max_res)
    }
    Some(max_res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
